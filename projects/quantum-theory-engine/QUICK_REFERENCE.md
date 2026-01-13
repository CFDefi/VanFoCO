# Quick Reference Guide

## Command Line Interface

### Build Commands
```bash
# Clean build
cargo clean && cargo build --release

# Run tests
cargo test --workspace

# Run benchmarks
cargo bench --workspace

# Generate documentation
cargo doc --open --no-deps

# Build Python bindings
cd python_bindings && maturin develop --release

# Production verification
./scripts/verify_production.sh
```

### CLI Usage (when implemented)
```bash
# Run simulation
quantum-cli simulate dsl_examples/rabi.phys --backend cpu

# Prove identity
quantum-cli prove "sigma_x^2 == I"

# Fit parameters
quantum-cli fit model.phys data.csv --param omega --param gamma

# Parameter sweep
quantum-cli sweep model.phys --param "omega:0.5:2.0:10" --workers 4

# Start job queue server
quantum-cli server --workers 8 --port 8080
```

## Python API

### Setup
```python
import quantum_theory_engine as qte
import numpy as np

# Configure logging
qte.set_log_level("info")
```

### Job Queue
```python
# Create queue
queue = qte.PyJobQueue(num_workers=4)

# Submit job
job = qte.PyJob(kind="simulate", program="rabi.phys", priority="high")
job.set_param("omega", 1.5)
job.set_param("T", 10.0)
job_id = queue.submit(job)

# Check status
status = queue.status(job_id)
print(f"Status: {status}")

# List all jobs
jobs = queue.list_jobs()
for jid, jstatus in jobs:
    print(f"{jid}: {jstatus}")
```

### Templates
```python
# Get registry
registry = qte.PyTemplateRegistry()

# List templates
templates = registry.list()
for tmpl in templates:
    print(f"{tmpl['name']}: {tmpl['description']}")

# Get specific template
rabi_template = registry.get("rabi")
print(rabi_template.get_code())

# Instantiate with parameters
code = registry.instantiate("rabi", {
    "omega": 2.0,
    "T": 15.0
})
print(code)
```

### Parameter Fitting
```python
# Load experimental data
data = qte.load_measurements("experiment.csv")
observed = data['observables']['sigma_z']['values']
uncertainties = data['observables']['sigma_z']['uncertainties']

# Define model function
def rabi_model(params):
    omega = params[0]
    times = np.linspace(0, 10, len(observed))
    return 0.5 * (1 - np.cos(omega * times))

# Fit parameters
result = qte.fit_mle(
    observed_values=list(observed),
    uncertainties=list(uncertainties),
    model_fn=rabi_model,
    initial_params=[1.0],
    max_iterations=100
)

# Print results
print(f"Best fit ω: {result['best_params'][0]:.3f} ± {result['uncertainties'][0]:.3f}")
print(f"Log-likelihood: {result['log_likelihood']:.2f}")
print(f"Converged: {result['converged']}")
print(f"Iterations: {result['iterations']}")
```

### Monitoring
```python
# Get performance metrics
metrics = qte.get_metrics()
for metric in metrics:
    print(f"{metric['name']}:")
    print(f"  Count: {metric['count']}")
    print(f"  Avg: {metric['avg_ms']} ms")
    print(f"  Max: {metric['max_ms']} ms")

# Get recent logs
logs = qte.get_logs(n=50)
for log in logs:
    print(f"[{log['level']}] {log['module']}: {log['message']}")

# Health check
health = qte.health_check()
print(f"Healthy: {health['healthy']}")
for check_name, check_result in health['checks'].items():
    status = "✓" if check_result['passed'] else "✗"
    print(f"  {status} {check_name}: {check_result['message']}")
```

## Rust API

### Job Queue
```rust
use quantum_theory_engine::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create job queue
    let queue = JobQueue::new(4);
    
    // Submit job
    let job = Job {
        id: uuid::Uuid::new_v4(),
        kind: JobKind::Simulate {
            program: std::fs::read_to_string("rabi.phys")?
        },
        priority: Priority::High,
        params: hashmap! {
            "omega".to_string() => 1.5,
            "T".to_string() => 10.0,
        },
        config: JobConfig::default(),
    };
    
    let job_id = queue.submit(job).await?;
    
    // Wait for completion
    loop {
        if let Some(result) = queue.get_result(job_id).await {
            println!("Job completed: {:?}", result);
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    Ok(())
}
```

### Streaming
```rust
use quantum_theory_engine::streaming::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut manager = StreamingManager::new();
    
    // Add CSV source
    manager.add_csv_source(
        "experiment_1".to_string(),
        PathBuf::from("data.csv")
    )?;
    
    manager.start_source("experiment_1")?;
    
    // Create rolling fit
    let mut rolling_fit = RollingFitEngine::new(1000, vec![1.0, 0.1]);
    
    // Process events
    while let Some(event) = manager.next_event().await {
        match event {
            StreamEvent::NewData { data, .. } => {
                rolling_fit.add_data(data);
                
                if rolling_fit.should_refit() {
                    let result = rolling_fit.refit()?;
                    println!("Updated params: {:?}", result.best_params);
                }
            }
            StreamEvent::SourceConnected { source_id } => {
                println!("Source connected: {}", source_id);
            }
            StreamEvent::SourceDisconnected { source_id, reason } => {
                println!("Source disconnected: {} ({})", source_id, reason);
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

### Templates
```rust
use quantum_theory_engine::templates::*;

fn main() -> Result<()> {
    let registry = TemplateRegistry::new();
    
    // List templates by category
    let single_qubit = registry.list_by_category(
        TemplateCategory::SingleQubit
    );
    
    for template in single_qubit {
        println!("{}: {}", template.name, template.description);
    }
    
    // Instantiate template
    let params = hashmap! {
        "omega".to_string() => 2.0,
        "T".to_string() => 15.0,
    };
    
    let code = registry.instantiate("rabi", &params)?;
    println!("Generated code:\n{}", code);
    
    Ok(())
}
```

### Statistics
```rust
use quantum_theory_engine::stats::*;

fn main() -> Result<()> {
    // Load data
    let data = load_measurements("experiment.csv")?;
    
    // Extract values
    let (observed, uncertainties) = &data.observables["sigma_z"];
    
    // Define likelihood function
    let likelihood_fn = |params: &[f64]| -> Result<f64> {
        let omega = params[0];
        let predicted: Vec<f64> = (0..observed.len())
            .map(|i| 0.5 * (1.0 - (omega * i as f64 * 0.1).cos()))
            .collect();
        
        gaussian_log_likelihood(observed, uncertainties, &predicted)
    };
    
    // Fit parameters
    let result = fit_parameters_mle(
        likelihood_fn,
        &[1.0],
        100
    )?;
    
    println!("Best fit: ω = {:.3} ± {:.3}", 
        result.best_params[0], 
        result.uncertainties[0]
    );
    println!("Log-likelihood: {:.2}", result.log_likelihood);
    
    Ok(())
}
```

### Logging
```rust
use quantum_theory_engine::logging::*;

fn main() {
    // Set log level
    set_log_level(LogLevel::Debug);
    
    // Log messages
    info("main", "Starting simulation".to_string());
    debug("main", format!("Parameters: ω={}", 1.5));
    
    // Use timer
    {
        let _timer = Timer::new("simulation");
        // ... do work ...
    } // Timer records metric automatically
    
    // Get metrics
    for metric in get_metrics() {
        println!("{}: count={}, avg={:?}", 
            metric.name, 
            metric.count, 
            metric.avg_duration
        );
    }
    
    // Get recent logs
    for log in get_recent_logs(10) {
        println!("[{:?}] {}: {}", 
            log.level, 
            log.module, 
            log.message
        );
    }
}
```

### Prover
```rust
use quantum_theory_engine::prover::*;

fn main() -> Result<()> {
    let mut prover = Prover::new();
    
    // Add assumptions
    prover.add_assumption(Assumption::Hermitian("H".to_string()));
    
    // Prove identity (stub - needs real expressions)
    // let result = prover.prove_identity(expr1, expr2)?;
    
    // if result.proven {
    //     println!("Proof found in {} steps", result.proof.steps.len());
    //     println!("Certificate: {}", result.certificate.hash);
    // }
    
    Ok(())
}
```

## CSV Format

### Measurement Data Format
```csv
# Observable, Value, Uncertainty
sigma_z, 0.95, 0.01
sigma_z, 0.93, 0.01
sigma_x, -0.05, 0.02
sigma_y, 0.02, 0.02
sigma_z, 0.94, 0.01
```

### Time-Series Format
```csv
# Time, Observable, Outcome, Count
0.0, sigma_z, 0, 485
0.0, sigma_z, 1, 515
1.0, sigma_z, 0, 320
1.0, sigma_z, 1, 680
2.0, sigma_z, 0, 150
2.0, sigma_z, 1, 850
```

## Environment Variables

```bash
# Logging
export RUST_LOG=debug
export QTE_LOG_LEVEL=info

# Performance
export QTE_NUM_WORKERS=8
export QTE_THREAD_POOL_SIZE=16

# Paths
export QTE_TEMPLATE_DIR=/path/to/templates
export QTE_DATA_DIR=/path/to/data
```

## Common Patterns

### Batch Processing
```python
queue = qte.PyJobQueue(num_workers=8)
job_ids = []

for param_value in parameter_range:
    job = qte.PyJob(kind="simulate", program="model.phys")
    job.set_param("param_name", param_value)
    job_ids.append(queue.submit(job))

# Wait for all jobs
results = []
for job_id in job_ids:
    result = wait_for_job(queue, job_id)
    results.append(result)
```

### Error Handling
```python
try:
    result = qte.fit_mle(...)
except RuntimeError as e:
    print(f"Fitting failed: {e}")
    # Fallback or retry logic
```

### Performance Monitoring
```rust
let _timer = Timer::new("critical_section");

// ... critical code ...

// Timer automatically records when dropped
```

## Troubleshooting

### Build Issues
```bash
# Missing dependencies
sudo apt-get install libopenblas-dev liblapack-dev

# Update Rust
rustup update stable

# Clean rebuild
cargo clean && cargo build --release
```

### Python Issues
```bash
# Rebuild bindings
cd python_bindings
maturin develop --release

# Check installation
python -c "import quantum_theory_engine; print('OK')"
```

### Performance Issues
```rust
// Enable optimizations
set_log_level(LogLevel::Warn); // Reduce logging overhead

// Increase worker count
let queue = JobQueue::new(num_cpus::get());

// Use release build
cargo build --release
```
