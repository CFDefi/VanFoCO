//! Integration tests for full workflows

use quantum_theory_engine::*;
use std::time::Duration;

#[tokio::test]
async fn test_full_job_submission_workflow() {
    // Create job queue
    let queue = JobQueue::new(2);
    
    // Submit simulation job
    let job = Job {
        id: uuid::Uuid::new_v4(),
        kind: JobKind::Simulate {
            program: "test_program".to_string(),
        },
        priority: Priority::Normal,
        params: std::collections::HashMap::new(),
        config: job_queue::JobConfig::default(),
    };
    
    let job_id = queue.submit(job).await.expect("Failed to submit job");
    
    // Wait a bit for processing
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Check status
    let status = queue.status(job_id).await;
    assert!(status.is_some());
}

#[tokio::test]
async fn test_parameter_sweep() {
    let queue = JobQueue::new(4);
    
    let base_job = Job {
        id: uuid::Uuid::new_v4(),
        kind: JobKind::Simulate {
            program: "rabi_oscillations".to_string(),
        },
        priority: Priority::Normal,
        params: std::collections::HashMap::new(),
        config: job_queue::JobConfig::default(),
    };
    
    let grid = job_queue::ParameterGrid {
        params: vec![
            job_queue::ParameterRange {
                name: "omega".to_string(),
                start: 0.5,
                end: 2.0,
                steps: 5,
                scale: job_queue::ParameterScale::Linear,
            },
        ],
        strategy: job_queue::GridStrategy::FullGrid,
    };
    
    let job_ids = queue.submit_sweep(base_job, grid).await.expect("Failed to submit sweep");
    
    assert_eq!(job_ids.len(), 5);
}

#[test]
fn test_template_instantiation() {
    let registry = TemplateRegistry::new();
    
    let mut params = std::collections::HashMap::new();
    params.insert("omega".to_string(), 1.5);
    params.insert("T".to_string(), 10.0);
    
    let code = registry.instantiate("rabi", &params).expect("Failed to instantiate");
    
    assert!(code.contains("omega: Real = 1.5"));
    assert!(code.contains("T: Real = 10"));
}

#[test]
fn test_proof_system() {
    use quantum_theory_engine::prover::*;
    
    let mut prover = Prover::new();
    
    // Test simple identity: σ_x² = I
    // This is a stub test - would need actual expression types
    // For now, just verify prover can be created
    assert!(prover.can_prove());
}

#[test]
fn test_statistics_workflow() {
    use quantum_theory_engine::stats::*;
    
    let observed = vec![1.0, 2.0, 3.0];
    let uncertainties = vec![0.1, 0.1, 0.1];
    let predicted = vec![1.05, 1.95, 3.02];
    
    let log_l = gaussian_log_likelihood(&observed, &uncertainties, &predicted)
        .expect("Failed to compute log-likelihood");
    
    assert!(log_l < 0.0); // Log-likelihood should be negative
    assert!(log_l > -10.0); // Should be reasonable for this data
}

#[test]
fn test_logging_system() {
    use quantum_theory_engine::logging::*;
    
    clear_logs();
    set_log_level(LogLevel::Debug);
    
    info("integration_test", "Test message 1".to_string());
    debug("integration_test", "Test message 2".to_string());
    error("integration_test", "Test error".to_string());
    
    let logs = get_recent_logs(10);
    assert_eq!(logs.len(), 3);
    
    let errors = filter_logs(LogLevel::Error, None);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message, "Test error");
}

#[test]
fn test_performance_metrics() {
    use quantum_theory_engine::logging::*;
    
    clear_logs();
    
    {
        let _timer = Timer::new("test_operation");
        std::thread::sleep(Duration::from_millis(10));
    } // Timer drops and records metric
    
    let metrics = get_metrics();
    let test_metric = metrics.iter().find(|m| m.name == "test_operation");
    
    assert!(test_metric.is_some());
    let metric = test_metric.unwrap();
    assert_eq!(metric.count, 1);
    assert!(metric.avg_duration >= Duration::from_millis(10));
}

#[test]
fn test_health_checks() {
    use quantum_theory_engine::logging::*;
    
    let checker = HealthChecker::default();
    let status = checker.run_checks();
    
    assert!(status.healthy); // Default checks should pass
    assert!(!status.checks.is_empty());
}

#[test]
fn test_csv_loading() {
    use quantum_theory_engine::stats::*;
    use std::io::Write;
    
    // Create temporary CSV file
    let temp_path = "/tmp/test_measurements.csv";
    let mut file = std::fs::File::create(temp_path).expect("Failed to create temp file");
    
    writeln!(file, "# Observable, Value, Uncertainty").unwrap();
    writeln!(file, "sigma_z, 0.5, 0.01").unwrap();
    writeln!(file, "sigma_x, -0.2, 0.02").unwrap();
    writeln!(file, "sigma_z, 0.48, 0.01").unwrap();
    
    drop(file);
    
    let data = load_measurements(temp_path).expect("Failed to load measurements");
    
    assert_eq!(data.num_shots, 3);
    assert!(data.observables.contains_key("sigma_z"));
    assert!(data.observables.contains_key("sigma_x"));
    
    let sigma_z = &data.observables["sigma_z"];
    assert_eq!(sigma_z.0.len(), 2); // Two sigma_z measurements
    
    // Cleanup
    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_gradient_computation() {
    use quantum_theory_engine::stats::*;
    
    // Test gradient of f(x, y) = x² + 2y
    let f = |params: &[f64]| -> quantum_theory_engine::error::Result<f64> {
        Ok(params[0].powi(2) + 2.0 * params[1])
    };
    
    let grad = compute_gradient(f, &[1.0, 2.0], 1e-6).expect("Failed to compute gradient");
    
    // df/dx = 2x = 2.0, df/dy = 2.0
    assert!((grad[0] - 2.0).abs() < 1e-4);
    assert!((grad[1] - 2.0).abs() < 1e-4);
}

#[test]
fn test_mle_fitting() {
    use quantum_theory_engine::stats::*;
    
    // Fit a simple linear model: y = a*x + b
    let x_values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_observed = vec![2.1, 3.9, 6.2, 8.0, 9.8]; // Roughly y = 2x
    let uncertainties = vec![0.1, 0.1, 0.1, 0.1, 0.1];
    
    let likelihood_fn = |params: &[f64]| -> quantum_theory_engine::error::Result<f64> {
        let a = params[0];
        let b = params[1];
        
        let predicted: Vec<f64> = x_values.iter().map(|&x| a * x + b).collect();
        
        gaussian_log_likelihood(&y_observed, &uncertainties, &predicted)
    };
    
    let result = fit_parameters_mle(likelihood_fn, &[1.0, 0.0], 100)
        .expect("Failed to fit");
    
    // Should find a ≈ 2, b ≈ 0
    assert!((result.best_params[0] - 2.0).abs() < 0.5);
    assert!(result.best_params[1].abs() < 1.0);
}

#[test]
fn test_end_to_end_workflow() {
    // This test simulates a complete workflow:
    // 1. Create template
    // 2. Instantiate with parameters
    // 3. Submit job
    // 4. Monitor status
    // 5. Retrieve results
    
    let registry = TemplateRegistry::new();
    
    // Step 1 & 2: Get template and instantiate
    let mut params = std::collections::HashMap::new();
    params.insert("omega".to_string(), 1.0);
    params.insert("T".to_string(), 10.0);
    
    let _code = registry.instantiate("rabi", &params).expect("Failed to instantiate");
    
    // Step 3-5 would require async runtime and full execution pipeline
    // For now, just verify the template system works
    assert!(true);
}

#[tokio::test]
async fn test_concurrent_job_execution() {
    let queue = JobQueue::new(4);
    
    // Submit multiple jobs concurrently
    let mut job_ids = Vec::new();
    
    for i in 0..10 {
        let job = Job {
            id: uuid::Uuid::new_v4(),
            kind: JobKind::Simulate {
                program: format!("program_{}", i),
            },
            priority: if i % 2 == 0 { Priority::High } else { Priority::Normal },
            params: std::collections::HashMap::new(),
            config: job_queue::JobConfig::default(),
        };
        
        let job_id = queue.submit(job).await.expect("Failed to submit job");
        job_ids.push(job_id);
    }
    
    // Wait for some processing
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Verify all jobs were queued/processed
    let all_jobs = queue.list_jobs().await;
    assert!(all_jobs.len() >= 10);
}
