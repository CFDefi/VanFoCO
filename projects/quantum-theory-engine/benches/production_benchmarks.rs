//! Performance benchmarks for critical paths

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use quantum_theory_engine::*;
use std::collections::HashMap;

fn benchmark_job_submission(c: &mut Criterion) {
    let mut group = c.benchmark_group("job_queue");
    
    for num_workers in [1, 2, 4, 8] {
        group.bench_with_input(
            BenchmarkId::new("submit", num_workers),
            &num_workers,
            |b, &workers| {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let queue = runtime.block_on(async {
                    JobQueue::new(workers)
                });
                
                b.iter(|| {
                    runtime.block_on(async {
                        let job = Job {
                            id: uuid::Uuid::new_v4(),
                            kind: job_queue::JobKind::Simulate {
                                program: "test".to_string(),
                            },
                            priority: job_queue::Priority::Normal,
                            params: HashMap::new(),
                            config: job_queue::JobConfig::default(),
                        };
                        
                        queue.submit(job).await.unwrap()
                    });
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_template_instantiation(c: &mut Criterion) {
    let registry = TemplateRegistry::new();
    let mut params = HashMap::new();
    params.insert("omega".to_string(), 1.0);
    params.insert("T".to_string(), 10.0);
    
    c.bench_function("template_instantiate", |b| {
        b.iter(|| {
            registry.instantiate(black_box("rabi"), black_box(&params)).unwrap()
        });
    });
}

fn benchmark_statistics(c: &mut Criterion) {
    let observed = vec![1.0; 1000];
    let uncertainties = vec![0.1; 1000];
    let predicted = vec![1.05; 1000];
    
    c.bench_function("gaussian_log_likelihood", |b| {
        b.iter(|| {
            stats::gaussian_log_likelihood(
                black_box(&observed),
                black_box(&uncertainties),
                black_box(&predicted),
            ).unwrap()
        });
    });
}

fn benchmark_gradient_computation(c: &mut Criterion) {
    let f = |params: &[f64]| -> error::Result<f64> {
        Ok(params[0].powi(2) + params[1].powi(2) + params[2].powi(2))
    };
    
    let params = vec![1.0, 2.0, 3.0];
    
    c.bench_function("compute_gradient", |b| {
        b.iter(|| {
            stats::compute_gradient(black_box(&f), black_box(&params), black_box(1e-6)).unwrap()
        });
    });
}

fn benchmark_logging(c: &mut Criterion) {
    logging::set_log_level(LogLevel::Info);
    
    c.bench_function("log_info", |b| {
        b.iter(|| {
            logging::info(black_box("benchmark"), black_box("Test message".to_string()));
        });
    });
    
    c.bench_function("timer_create_drop", |b| {
        b.iter(|| {
            let _timer = Timer::new(black_box("benchmark_timer"));
        });
    });
}

fn benchmark_proof_search(c: &mut Criterion) {
    // Benchmark prover performance
    c.bench_function("prover_create", |b| {
        b.iter(|| {
            prover::Prover::new()
        });
    });
}

fn benchmark_parameter_sweep_generation(c: &mut Criterion) {
    let grid = job_queue::ParameterGrid {
        params: vec![
            job_queue::ParameterRange {
                name: "x".to_string(),
                start: 0.0,
                end: 1.0,
                steps: 10,
                scale: job_queue::ParameterScale::Linear,
            },
            job_queue::ParameterRange {
                name: "y".to_string(),
                start: 0.1,
                end: 10.0,
                steps: 10,
                scale: job_queue::ParameterScale::Log,
            },
        ],
        strategy: job_queue::GridStrategy::FullGrid,
    };
    
    c.bench_function("generate_full_grid", |b| {
        b.iter(|| {
            job_queue::ParameterGrid {
                params: grid.params.clone(),
                strategy: job_queue::GridStrategy::FullGrid,
            }
        });
    });
}

fn benchmark_mle_fitting(c: &mut Criterion) {
    let x_values = (0..100).map(|i| i as f64 * 0.1).collect::<Vec<_>>();
    let y_observed: Vec<f64> = x_values.iter().map(|&x| 2.0 * x + 1.0 + (x * 0.1).sin() * 0.1).collect();
    let uncertainties = vec![0.1; 100];
    
    let likelihood_fn = |params: &[f64]| -> error::Result<f64> {
        let a = params[0];
        let b = params[1];
        let predicted: Vec<f64> = x_values.iter().map(|&x| a * x + b).collect();
        stats::gaussian_log_likelihood(&y_observed, &uncertainties, &predicted)
    };
    
    c.bench_function("mle_fit_linear", |b| {
        b.iter(|| {
            stats::fit_parameters_mle(
                black_box(&likelihood_fn),
                black_box(&[1.0, 0.0]),
                black_box(50),
            ).unwrap()
        });
    });
}

criterion_group!(
    benches,
    benchmark_job_submission,
    benchmark_template_instantiation,
    benchmark_statistics,
    benchmark_gradient_computation,
    benchmark_logging,
    benchmark_proof_search,
    benchmark_parameter_sweep_generation,
    benchmark_mle_fitting,
);

criterion_main!(benches);
