//! Job queue system for batch processing and parameter sweeps
//!
//! Provides asynchronous job execution with priority management,
//! progress tracking, and resource limits.

use crate::ast::Ast;
use crate::error::{EngineError, Result};
use crate::executor::{ExecutionResult, Executor, BackendConfig};
use crate::ir::IrProgram;
use crate::stats::MeasurementData;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use uuid::Uuid;

/// Job queue manager
pub struct JobQueue {
    sender: mpsc::UnboundedSender<JobCommand>,
    state: Arc<Mutex<QueueState>>,
    _workers: Vec<JoinHandle<()>>,
}

struct QueueState {
    jobs: HashMap<JobId, JobInfo>,
    queue: VecDeque<JobId>,
    running: HashMap<JobId, Instant>,
    results: HashMap<JobId, JobResult>,
}

pub type JobId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub kind: JobKind,
    pub priority: Priority,
    pub params: HashMap<String, f64>,
    pub config: JobConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobKind {
    Simulate { program: String },  // Serialized IR or DSL
    Prove { statement: String },
    Fit { program: String, data_path: String },
    Test { program: String, data_path: String },
    Sweep { base_job: Box<Job>, param_grid: ParameterGrid },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobConfig {
    pub timeout: Duration,
    pub max_retries: usize,
    pub backend: BackendConfig,
}

impl Default for JobConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(300),
            max_retries: 3,
            backend: BackendConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterGrid {
    pub params: Vec<ParameterRange>,
    pub strategy: GridStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterRange {
    pub name: String,
    pub start: f64,
    pub end: f64,
    pub steps: usize,
    pub scale: ParameterScale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterScale {
    Linear,
    Log,
    Custom(Vec<f64>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GridStrategy {
    FullGrid,     // Cartesian product
    Random(usize), // Random sampling
    Sobol(usize),  // Quasi-random Sobol sequence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Queued { position: usize },
    Running { progress: f64, eta: Option<Duration> },
    Complete(JobResult),
    Failed { error: String, retry_count: usize },
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    pub job_id: JobId,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub duration: Duration,
    pub output: JobOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobOutput {
    Simulation(SimulationOutput),
    Proof(ProofOutput),
    Fit(FitOutput),
    Test(TestOutput),
    Sweep(SweepOutput),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationOutput {
    pub states: Vec<Vec<f64>>,  // Simplified for serialization
    pub diagnostics: DiagnosticsSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOutput {
    pub proven: bool,
    pub steps: usize,
    pub certificate_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitOutput {
    pub best_params: Vec<f64>,
    pub uncertainties: Vec<f64>,
    pub log_likelihood: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOutput {
    pub chi_square: f64,
    pub p_value: f64,
    pub decision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweepOutput {
    pub results: Vec<(HashMap<String, f64>, JobResult)>,
    pub best_params: HashMap<String, f64>,
    pub summary: SweepSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsSummary {
    pub max_trace_drift: f64,
    pub min_eigenvalue: f64,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweepSummary {
    pub total_runs: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_duration: Duration,
}

struct JobInfo {
    job: Job,
    submitted_at: Instant,
    retry_count: usize,
}

enum JobCommand {
    Submit {
        job: Job,
        response: oneshot::Sender<JobId>,
    },
    Cancel {
        job_id: JobId,
        response: oneshot::Sender<Result<()>>,
    },
    GetStatus {
        job_id: JobId,
        response: oneshot::Sender<Option<JobStatus>>,
    },
    GetResult {
        job_id: JobId,
        response: oneshot::Sender<Option<JobResult>>,
    },
    ListJobs {
        response: oneshot::Sender<Vec<(JobId, JobStatus)>>,
    },
}

impl JobQueue {
    pub fn new(num_workers: usize) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let state = Arc::new(Mutex::new(QueueState {
            jobs: HashMap::new(),
            queue: VecDeque::new(),
            running: HashMap::new(),
            results: HashMap::new(),
        }));

        let workers = (0..num_workers)
            .map(|id| {
                let state = Arc::clone(&state);
                tokio::spawn(async move {
                    worker_loop(id, state).await;
                })
            })
            .collect();

        // Spawn command processor
        let cmd_state = Arc::clone(&state);
        tokio::spawn(async move {
            command_processor(receiver, cmd_state).await;
        });

        Self {
            sender,
            state,
            _workers: workers,
        }
    }

    pub async fn submit(&self, job: Job) -> Result<JobId> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(JobCommand::Submit { job, response: tx })
            .map_err(|_| EngineError::Internal("Job queue closed".to_string()))?;
        
        rx.await
            .map_err(|_| EngineError::Internal("Failed to receive job ID".to_string()))
    }

    pub async fn submit_sweep(&self, base_job: Job, grid: ParameterGrid) -> Result<Vec<JobId>> {
        let param_combinations = generate_parameter_combinations(&grid);
        let mut job_ids = Vec::new();

        for params in param_combinations {
            let mut job = base_job.clone();
            job.id = Uuid::new_v4();
            job.params = params;
            
            let job_id = self.submit(job).await?;
            job_ids.push(job_id);
        }

        Ok(job_ids)
    }

    pub async fn cancel(&self, job_id: JobId) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(JobCommand::Cancel { job_id, response: tx })
            .map_err(|_| EngineError::Internal("Job queue closed".to_string()))?;
        
        rx.await
            .map_err(|_| EngineError::Internal("Failed to receive cancel response".to_string()))?
    }

    pub async fn status(&self, job_id: JobId) -> Option<JobStatus> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(JobCommand::GetStatus { job_id, response: tx })
            .ok()?;
        
        rx.await.ok()?
    }

    pub async fn get_result(&self, job_id: JobId) -> Option<JobResult> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(JobCommand::GetResult { job_id, response: tx })
            .ok()?;
        
        rx.await.ok()?
    }

    pub async fn list_jobs(&self) -> Vec<(JobId, JobStatus)> {
        let (tx, rx) = oneshot::channel();
        if self.sender.send(JobCommand::ListJobs { response: tx }).is_err() {
            return vec![];
        }
        
        rx.await.unwrap_or_default()
    }

    pub async fn aggregate_sweep(&self, job_ids: &[JobId]) -> Result<SweepOutput> {
        let mut results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        let start = Instant::now();

        for job_id in job_ids {
            if let Some(result) = self.get_result(*job_id).await {
                let params = self.state.lock().unwrap()
                    .jobs.get(job_id)
                    .map(|info| info.job.params.clone())
                    .unwrap_or_default();
                
                results.push((params, result));
                successful += 1;
            } else {
                failed += 1;
            }
        }

        // Find best parameters (highest log-likelihood)
        let best_params = results.iter()
            .filter_map(|(params, result)| {
                if let JobOutput::Fit(fit) = &result.output {
                    Some((params.clone(), fit.log_likelihood))
                } else {
                    None
                }
            })
            .max_by(|(_, ll1), (_, ll2)| ll1.partial_cmp(ll2).unwrap())
            .map(|(params, _)| params)
            .unwrap_or_default();

        Ok(SweepOutput {
            results,
            best_params,
            summary: SweepSummary {
                total_runs: job_ids.len(),
                successful,
                failed,
                total_duration: start.elapsed(),
            },
        })
    }
}

async fn command_processor(
    mut receiver: mpsc::UnboundedReceiver<JobCommand>,
    state: Arc<Mutex<QueueState>>,
) {
    while let Some(cmd) = receiver.recv().await {
        match cmd {
            JobCommand::Submit { job, response } => {
                let job_id = job.id;
                let mut state = state.lock().unwrap();
                
                state.jobs.insert(job_id, JobInfo {
                    job: job.clone(),
                    submitted_at: Instant::now(),
                    retry_count: 0,
                });
                
                // Insert into priority queue
                let insert_pos = state.queue.iter()
                    .position(|id| {
                        state.jobs.get(id).map(|j| j.job.priority).unwrap_or(Priority::Low)
                            < job.priority
                    })
                    .unwrap_or(state.queue.len());
                
                state.queue.insert(insert_pos, job_id);
                
                let _ = response.send(job_id);
            }

            JobCommand::Cancel { job_id, response } => {
                let mut state = state.lock().unwrap();
                
                // Remove from queue if not running
                state.queue.retain(|id| *id != job_id);
                state.running.remove(&job_id);
                
                let _ = response.send(Ok(()));
            }

            JobCommand::GetStatus { job_id, response } => {
                let state = state.lock().unwrap();
                
                let status = if let Some(result) = state.results.get(&job_id) {
                    Some(JobStatus::Complete(result.clone()))
                } else if state.running.contains_key(&job_id) {
                    Some(JobStatus::Running { progress: 0.5, eta: None })
                } else if let Some(pos) = state.queue.iter().position(|id| *id == job_id) {
                    Some(JobStatus::Queued { position: pos })
                } else {
                    None
                };
                
                let _ = response.send(status);
            }

            JobCommand::GetResult { job_id, response } => {
                let state = state.lock().unwrap();
                let _ = response.send(state.results.get(&job_id).cloned());
            }

            JobCommand::ListJobs { response } => {
                let state = state.lock().unwrap();
                let jobs: Vec<_> = state.jobs.keys()
                    .filter_map(|id| {
                        let status = if state.results.contains_key(id) {
                            JobStatus::Complete(state.results[id].clone())
                        } else if state.running.contains_key(id) {
                            JobStatus::Running { progress: 0.5, eta: None }
                        } else {
                            JobStatus::Queued { 
                                position: state.queue.iter().position(|qid| qid == id).unwrap_or(0) 
                            }
                        };
                        Some((*id, status))
                    })
                    .collect();
                
                let _ = response.send(jobs);
            }
        }
    }
}

async fn worker_loop(worker_id: usize, state: Arc<Mutex<QueueState>>) {
    loop {
        // Get next job from queue
        let job_info = {
            let mut state = state.lock().unwrap();
            if let Some(job_id) = state.queue.pop_front() {
                state.running.insert(job_id, Instant::now());
                state.jobs.get(&job_id).cloned()
            } else {
                None
            }
        };

        if let Some(info) = job_info {
            println!("Worker {} executing job {}", worker_id, info.job.id);
            
            let result = execute_job(&info.job).await;
            
            let mut state = state.lock().unwrap();
            state.running.remove(&info.job.id);
            
            match result {
                Ok(output) => {
                    state.results.insert(info.job.id, JobResult {
                        job_id: info.job.id,
                        started_at: chrono::Utc::now(),
                        completed_at: chrono::Utc::now(),
                        duration: info.submitted_at.elapsed(),
                        output,
                    });
                }
                Err(e) => {
                    println!("Job {} failed: {:?}", info.job.id, e);
                    // Could implement retry logic here
                }
            }
        } else {
            // No jobs available, sleep briefly
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

async fn execute_job(job: &Job) -> Result<JobOutput> {
    // This is a stub - actual implementation would:
    // 1. Parse DSL or deserialize IR
    // 2. Run through pipeline
    // 3. Execute on backend
    // 4. Return results
    
    match &job.kind {
        JobKind::Simulate { .. } => {
            Ok(JobOutput::Simulation(SimulationOutput {
                states: vec![],
                diagnostics: DiagnosticsSummary {
                    max_trace_drift: 1e-10,
                    min_eigenvalue: -1e-11,
                    warnings: vec![],
                },
            }))
        }
        JobKind::Prove { .. } => {
            Ok(JobOutput::Proof(ProofOutput {
                proven: true,
                steps: 5,
                certificate_hash: Some("abc123".to_string()),
            }))
        }
        JobKind::Fit { .. } => {
            Ok(JobOutput::Fit(FitOutput {
                best_params: vec![1.0, 0.2],
                uncertainties: vec![0.01, 0.02],
                log_likelihood: -100.0,
            }))
        }
        JobKind::Test { .. } => {
            Ok(JobOutput::Test(TestOutput {
                chi_square: 45.3,
                p_value: 0.42,
                decision: "Accept".to_string(),
            }))
        }
        JobKind::Sweep { .. } => {
            Err(EngineError::Unsupported("Nested sweeps not supported".to_string()))
        }
    }
}

fn generate_parameter_combinations(grid: &ParameterGrid) -> Vec<HashMap<String, f64>> {
    match grid.strategy {
        GridStrategy::FullGrid => generate_full_grid(&grid.params),
        GridStrategy::Random(n) => generate_random_grid(&grid.params, n),
        GridStrategy::Sobol(n) => generate_sobol_grid(&grid.params, n),
    }
}

fn generate_full_grid(params: &[ParameterRange]) -> Vec<HashMap<String, f64>> {
    if params.is_empty() {
        return vec![HashMap::new()];
    }

    let first = &params[0];
    let rest = &params[1..];
    
    let values = generate_param_values(first);
    let rest_combinations = generate_full_grid(rest);
    
    let mut result = Vec::new();
    for value in values {
        for combo in &rest_combinations {
            let mut new_combo = combo.clone();
            new_combo.insert(first.name.clone(), value);
            result.push(new_combo);
        }
    }
    
    result
}

fn generate_param_values(param: &ParameterRange) -> Vec<f64> {
    match &param.scale {
        ParameterScale::Linear => {
            (0..param.steps)
                .map(|i| {
                    let t = i as f64 / (param.steps - 1).max(1) as f64;
                    param.start + t * (param.end - param.start)
                })
                .collect()
        }
        ParameterScale::Log => {
            let log_start = param.start.ln();
            let log_end = param.end.ln();
            (0..param.steps)
                .map(|i| {
                    let t = i as f64 / (param.steps - 1).max(1) as f64;
                    (log_start + t * (log_end - log_start)).exp()
                })
                .collect()
        }
        ParameterScale::Custom(values) => values.clone(),
    }
}

fn generate_random_grid(params: &[ParameterRange], n: usize) -> Vec<HashMap<String, f64>> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    (0..n).map(|_| {
        params.iter().map(|param| {
            let value = match &param.scale {
                ParameterScale::Linear => {
                    rng.gen_range(param.start..=param.end)
                }
                ParameterScale::Log => {
                    let log_start = param.start.ln();
                    let log_end = param.end.ln();
                    rng.gen_range(log_start..=log_end).exp()
                }
                ParameterScale::Custom(values) => {
                    values[rng.gen_range(0..values.len())]
                }
            };
            (param.name.clone(), value)
        }).collect()
    }).collect()
}

fn generate_sobol_grid(_params: &[ParameterRange], _n: usize) -> Vec<HashMap<String, f64>> {
    // TODO: Implement Sobol quasi-random sequence
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_job_submission() {
        let queue = JobQueue::new(2);
        
        let job = Job {
            id: Uuid::new_v4(),
            kind: JobKind::Simulate { program: "test".to_string() },
            priority: Priority::Normal,
            params: HashMap::new(),
            config: JobConfig::default(),
        };

        let job_id = queue.submit(job).await.unwrap();
        assert!(!job_id.is_nil());
    }

    #[test]
    fn test_parameter_grid() {
        let grid = ParameterGrid {
            params: vec![
                ParameterRange {
                    name: "x".to_string(),
                    start: 0.0,
                    end: 1.0,
                    steps: 3,
                    scale: ParameterScale::Linear,
                },
                ParameterRange {
                    name: "y".to_string(),
                    start: 1.0,
                    end: 10.0,
                    steps: 2,
                    scale: ParameterScale::Log,
                },
            ],
            strategy: GridStrategy::FullGrid,
        };

        let combos = generate_parameter_combinations(&grid);
        assert_eq!(combos.len(), 6); // 3 × 2
    }
}
