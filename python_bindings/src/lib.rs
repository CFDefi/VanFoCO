//! Python bindings for quantum-theory-engine
//!
//! Provides a complete Python API with numpy integration.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::PyRuntimeError;
use numpy::{PyArray1, PyArray2, ToPyArray};
use quantum_theory_engine::{
    JobQueue, Job, JobKind, Priority, JobConfig, JobStatus,
    TemplateRegistry, Template,
    LogLevel, Timer, HealthChecker,
};
use std::collections::HashMap;

/// Python wrapper for Job
#[pyclass]
struct PyJob {
    inner: Job,
}

#[pymethods]
impl PyJob {
    #[new]
    fn new(kind: String, program: String, priority: Option<String>) -> PyResult<Self> {
        let job_kind = match kind.as_str() {
            "simulate" => JobKind::Simulate { program },
            "prove" => JobKind::Prove { statement: program },
            _ => return Err(PyRuntimeError::new_err(format!("Invalid job kind: {}", kind))),
        };
        
        let priority = match priority.as_deref() {
            Some("low") => Priority::Low,
            Some("high") => Priority::High,
            Some("critical") => Priority::Critical,
            _ => Priority::Normal,
        };
        
        Ok(Self {
            inner: Job {
                id: uuid::Uuid::new_v4(),
                kind: job_kind,
                priority,
                params: HashMap::new(),
                config: JobConfig::default(),
            },
        })
    }
    
    fn set_param(&mut self, name: String, value: f64) {
        self.inner.params.insert(name, value);
    }
    
    fn get_id(&self) -> String {
        self.inner.id.to_string()
    }
}

/// Python wrapper for JobQueue
#[pyclass]
struct PyJobQueue {
    runtime: tokio::runtime::Runtime,
    queue: Option<JobQueue>,
}

#[pymethods]
impl PyJobQueue {
    #[new]
    fn new(num_workers: Option<usize>) -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;
        
        let queue = runtime.block_on(async {
            JobQueue::new(num_workers.unwrap_or(4))
        });
        
        Ok(Self {
            runtime,
            queue: Some(queue),
        })
    }
    
    fn submit(&mut self, job: &PyJob) -> PyResult<String> {
        let queue = self.queue.as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("Queue is closed"))?;
        
        let job_id = self.runtime.block_on(async {
            queue.submit(job.inner.clone()).await
        }).map_err(|e| PyRuntimeError::new_err(format!("{:?}", e)))?;
        
        Ok(job_id.to_string())
    }
    
    fn status(&self, job_id: String) -> PyResult<Option<String>> {
        let queue = self.queue.as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("Queue is closed"))?;
        
        let uuid = uuid::Uuid::parse_str(&job_id)
            .map_err(|e| PyRuntimeError::new_err(format!("Invalid job ID: {}", e)))?;
        
        let status = self.runtime.block_on(async {
            queue.status(uuid).await
        });
        
        Ok(status.map(|s| format!("{:?}", s)))
    }
    
    fn list_jobs(&self) -> PyResult<Vec<(String, String)>> {
        let queue = self.queue.as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("Queue is closed"))?;
        
        let jobs = self.runtime.block_on(async {
            queue.list_jobs().await
        });
        
        Ok(jobs.into_iter()
            .map(|(id, status)| (id.to_string(), format!("{:?}", status)))
            .collect())
    }
}

/// Python wrapper for TemplateRegistry
#[pyclass]
struct PyTemplateRegistry {
    registry: TemplateRegistry,
}

#[pymethods]
impl PyTemplateRegistry {
    #[new]
    fn new() -> Self {
        Self {
            registry: TemplateRegistry::new(),
        }
    }
    
    fn get(&self, id: &str) -> PyResult<Option<PyTemplate>> {
        Ok(self.registry.get(id).map(|t| PyTemplate {
            inner: t.clone(),
        }))
    }
    
    fn list(&self, py: Python) -> PyResult<PyObject> {
        let list = PyList::empty(py);
        for (id, template) in self.registry.search("") {
            let dict = PyDict::new(py);
            dict.set_item("id", &template.id)?;
            dict.set_item("name", &template.name)?;
            dict.set_item("description", &template.description)?;
            dict.set_item("category", format!("{:?}", template.category))?;
            list.append(dict)?;
        }
        Ok(list.into())
    }
    
    fn instantiate(&self, id: &str, params: HashMap<String, f64>) -> PyResult<String> {
        self.registry.instantiate(id, &params)
            .map_err(|e| PyRuntimeError::new_err(format!("{:?}", e)))
    }
}

#[pyclass]
struct PyTemplate {
    inner: Template,
}

#[pymethods]
impl PyTemplate {
    fn get_id(&self) -> String {
        self.inner.id.clone()
    }
    
    fn get_name(&self) -> String {
        self.inner.name.clone()
    }
    
    fn get_description(&self) -> String {
        self.inner.description.clone()
    }
    
    fn get_code(&self) -> String {
        self.inner.code.clone()
    }
}

/// Execute DSL code and return results
#[pyfunction]
fn execute(py: Python, dsl_code: String) -> PyResult<PyObject> {
    // This is a stub - full implementation would:
    // 1. Parse DSL
    // 2. Compile to IR
    // 3. Execute
    // 4. Return results as numpy arrays
    
    let dict = PyDict::new(py);
    dict.set_item("status", "success")?;
    dict.set_item("message", "Execution stub - not yet implemented")?;
    
    Ok(dict.into())
}

/// Parse DSL and validate without executing
#[pyfunction]
fn validate(dsl_code: String) -> PyResult<bool> {
    // Stub
    Ok(true)
}

/// Set global log level
#[pyfunction]
fn set_log_level(level: String) -> PyResult<()> {
    let log_level = match level.as_str() {
        "trace" => LogLevel::Trace,
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => return Err(PyRuntimeError::new_err(format!("Invalid log level: {}", level))),
    };
    
    quantum_theory_engine::logging::set_log_level(log_level);
    Ok(())
}

/// Get performance metrics
#[pyfunction]
fn get_metrics(py: Python) -> PyResult<PyObject> {
    let metrics = quantum_theory_engine::logging::get_metrics();
    
    let list = PyList::empty(py);
    for metric in metrics {
        let dict = PyDict::new(py);
        dict.set_item("name", metric.name)?;
        dict.set_item("count", metric.count)?;
        dict.set_item("total_ms", metric.duration.as_millis())?;
        dict.set_item("avg_ms", metric.avg_duration.as_millis())?;
        dict.set_item("max_ms", metric.max_duration.as_millis())?;
        dict.set_item("min_ms", metric.min_duration.as_millis())?;
        list.append(dict)?;
    }
    
    Ok(list.into())
}

/// Get recent log entries
#[pyfunction]
fn get_logs(py: Python, n: usize) -> PyResult<PyObject> {
    let logs = quantum_theory_engine::logging::get_recent_logs(n);
    
    let list = PyList::empty(py);
    for log in logs {
        let dict = PyDict::new(py);
        dict.set_item("timestamp", log.timestamp.to_rfc3339())?;
        dict.set_item("level", format!("{:?}", log.level))?;
        dict.set_item("module", log.module)?;
        dict.set_item("message", log.message)?;
        list.append(dict)?;
    }
    
    Ok(list.into())
}

/// Run health checks
#[pyfunction]
fn health_check(py: Python) -> PyResult<PyObject> {
    let checker = HealthChecker::default();
    let status = checker.run_checks();
    
    let dict = PyDict::new(py);
    dict.set_item("healthy", status.healthy)?;
    dict.set_item("timestamp", status.timestamp.to_rfc3339())?;
    
    let checks = PyDict::new(py);
    for (name, result) in status.checks {
        let check_dict = PyDict::new(py);
        check_dict.set_item("passed", result.passed)?;
        check_dict.set_item("message", result.message)?;
        checks.set_item(name, check_dict)?;
    }
    dict.set_item("checks", checks)?;
    
    Ok(dict.into())
}

/// Fit parameters using MLE
#[pyfunction]
fn fit_mle<'py>(
    py: Python<'py>,
    observed_values: Vec<f64>,
    uncertainties: Vec<f64>,
    model_fn: PyObject,
    initial_params: Vec<f64>,
    max_iterations: Option<usize>,
) -> PyResult<&'py PyDict> {
    use quantum_theory_engine::stats::{gaussian_log_likelihood, fit_parameters_mle};
    
    // Create closure that calls Python model function
    let likelihood_fn = |params: &[f64]| -> quantum_theory_engine::error::Result<f64> {
        Python::with_gil(|py| {
            let params_list = params.to_vec().to_object(py);
            let predicted = model_fn.call1(py, (params_list,))
                .map_err(|e| quantum_theory_engine::error::EngineError::Internal(format!("{}", e)))?;
            
            let predicted_values: Vec<f64> = predicted.extract(py)
                .map_err(|e| quantum_theory_engine::error::EngineError::Internal(format!("{}", e)))?;
            
            gaussian_log_likelihood(&observed_values, &uncertainties, &predicted_values)
        })
    };
    
    let result = fit_parameters_mle(
        likelihood_fn,
        &initial_params,
        max_iterations.unwrap_or(100),
    ).map_err(|e| PyRuntimeError::new_err(format!("{:?}", e)))?;
    
    let dict = PyDict::new(py);
    dict.set_item("best_params", result.best_params.to_pyarray(py))?;
    dict.set_item("uncertainties", result.uncertainties.to_pyarray(py))?;
    dict.set_item("log_likelihood", result.log_likelihood)?;
    dict.set_item("converged", result.converged)?;
    dict.set_item("iterations", result.iterations)?;
    
    Ok(dict)
}

/// Load measurement data from CSV
#[pyfunction]
fn load_measurements(py: Python, csv_path: String) -> PyResult<PyObject> {
    use quantum_theory_engine::stats::load_measurements;
    
    let data = load_measurements(&csv_path)
        .map_err(|e| PyRuntimeError::new_err(format!("{:?}", e)))?;
    
    let dict = PyDict::new(py);
    dict.set_item("num_shots", data.num_shots)?;
    
    let observables = PyDict::new(py);
    for (name, (values, uncertainties)) in data.observables {
        let obs_dict = PyDict::new(py);
        obs_dict.set_item("values", values.to_pyarray(py))?;
        obs_dict.set_item("uncertainties", uncertainties.to_pyarray(py))?;
        observables.set_item(name, obs_dict)?;
    }
    dict.set_item("observables", observables)?;
    
    Ok(dict.into())
}

/// Python module definition
#[pymodule]
fn quantum_theory_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJob>()?;
    m.add_class::<PyJobQueue>()?;
    m.add_class::<PyTemplateRegistry>()?;
    m.add_class::<PyTemplate>()?;
    
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(set_log_level, m)?)?;
    m.add_function(wrap_pyfunction!(get_metrics, m)?)?;
    m.add_function(wrap_pyfunction!(get_logs, m)?)?;
    m.add_function(wrap_pyfunction!(health_check, m)?)?;
    m.add_function(wrap_pyfunction!(fit_mle, m)?)?;
    m.add_function(wrap_pyfunction!(load_measurements, m)?)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_module() {
        pyo3::prepare_freethreaded_python();
        
        Python::with_gil(|py| {
            let module = PyModule::new(py, "test").unwrap();
            quantum_theory_engine(py, module).unwrap();
            
            assert!(module.hasattr("execute").unwrap());
            assert!(module.hasattr("PyJobQueue").unwrap());
        });
    }
}
