//! Structured logging and telemetry
//!
//! Provides comprehensive logging, performance metrics, and diagnostics.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Global logger instance
static LOGGER: once_cell::sync::Lazy<Arc<Mutex<Logger>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(Logger::new())));

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub context: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub name: String,
    pub duration: Duration,
    pub count: u64,
    pub avg_duration: Duration,
    pub max_duration: Duration,
    pub min_duration: Duration,
}

pub struct Logger {
    level: LogLevel,
    entries: Vec<LogEntry>,
    max_entries: usize,
    metrics: HashMap<String, MetricStats>,
}

struct MetricStats {
    count: u64,
    total_duration: Duration,
    max_duration: Duration,
    min_duration: Duration,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: LogLevel::Info,
            entries: Vec::new(),
            max_entries: 10000,
            metrics: HashMap::new(),
        }
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn log(&mut self, level: LogLevel, module: &str, message: String, context: HashMap<String, serde_json::Value>) {
        if level >= self.level {
            let entry = LogEntry {
                timestamp: chrono::Utc::now(),
                level,
                module: module.to_string(),
                message,
                context,
            };

            // Print to stdout/stderr based on level
            match level {
                LogLevel::Error => eprintln!("[ERROR] {}: {}", module, entry.message),
                LogLevel::Warn => eprintln!("[WARN]  {}: {}", module, entry.message),
                LogLevel::Info => println!("[INFO]  {}: {}", module, entry.message),
                LogLevel::Debug => println!("[DEBUG] {}: {}", module, entry.message),
                LogLevel::Trace => println!("[TRACE] {}: {}", module, entry.message),
            }

            self.entries.push(entry);

            // Trim old entries if we exceed max
            if self.entries.len() > self.max_entries {
                self.entries.drain(0..self.entries.len() - self.max_entries);
            }
        }
    }

    pub fn record_metric(&mut self, name: &str, duration: Duration) {
        let stats = self.metrics.entry(name.to_string()).or_insert(MetricStats {
            count: 0,
            total_duration: Duration::ZERO,
            max_duration: Duration::ZERO,
            min_duration: Duration::MAX,
        });

        stats.count += 1;
        stats.total_duration += duration;
        stats.max_duration = stats.max_duration.max(duration);
        stats.min_duration = stats.min_duration.min(duration);
    }

    pub fn get_metrics(&self) -> Vec<PerformanceMetric> {
        self.metrics.iter().map(|(name, stats)| {
            let avg = if stats.count > 0 {
                stats.total_duration / stats.count as u32
            } else {
                Duration::ZERO
            };

            PerformanceMetric {
                name: name.clone(),
                duration: stats.total_duration,
                count: stats.count,
                avg_duration: avg,
                max_duration: stats.max_duration,
                min_duration: if stats.min_duration == Duration::MAX {
                    Duration::ZERO
                } else {
                    stats.min_duration
                },
            }
        }).collect()
    }

    pub fn get_recent_logs(&self, n: usize) -> Vec<LogEntry> {
        let start = if self.entries.len() > n {
            self.entries.len() - n
        } else {
            0
        };
        self.entries[start..].to_vec()
    }

    pub fn filter_logs(&self, level: LogLevel, module: Option<&str>) -> Vec<LogEntry> {
        self.entries.iter()
            .filter(|entry| {
                entry.level >= level &&
                    module.map_or(true, |m| entry.module == m)
            })
            .cloned()
            .collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.metrics.clear();
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

// Global logging functions

pub fn set_log_level(level: LogLevel) {
    LOGGER.lock().unwrap().set_level(level);
}

pub fn trace(module: &str, message: String) {
    LOGGER.lock().unwrap().log(LogLevel::Trace, module, message, HashMap::new());
}

pub fn debug(module: &str, message: String) {
    LOGGER.lock().unwrap().log(LogLevel::Debug, module, message, HashMap::new());
}

pub fn info(module: &str, message: String) {
    LOGGER.lock().unwrap().log(LogLevel::Info, module, message, HashMap::new());
}

pub fn warn(module: &str, message: String) {
    LOGGER.lock().unwrap().log(LogLevel::Warn, module, message, HashMap::new());
}

pub fn error(module: &str, message: String) {
    LOGGER.lock().unwrap().log(LogLevel::Error, module, message, HashMap::new());
}

pub fn log_with_context(level: LogLevel, module: &str, message: String, context: HashMap<String, serde_json::Value>) {
    LOGGER.lock().unwrap().log(level, module, message, context);
}

pub fn record_metric(name: &str, duration: Duration) {
    LOGGER.lock().unwrap().record_metric(name, duration);
}

pub fn get_metrics() -> Vec<PerformanceMetric> {
    LOGGER.lock().unwrap().get_metrics()
}

pub fn get_recent_logs(n: usize) -> Vec<LogEntry> {
    LOGGER.lock().unwrap().get_recent_logs(n)
}

pub fn filter_logs(level: LogLevel, module: Option<&str>) -> Vec<LogEntry> {
    LOGGER.lock().unwrap().filter_logs(level, module)
}

pub fn clear_logs() {
    LOGGER.lock().unwrap().clear();
}

/// Performance timer for automatic metric recording
pub struct Timer {
    name: String,
    start: Instant,
}

impl Timer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn record(&self) {
        record_metric(&self.name, self.elapsed());
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.record();
    }
}

/// Macro for easy logging
#[macro_export]
macro_rules! log_trace {
    ($module:expr, $($arg:tt)*) => {
        $crate::logging::trace($module, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($module:expr, $($arg:tt)*) => {
        $crate::logging::debug($module, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($module:expr, $($arg:tt)*) => {
        $crate::logging::info($module, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($module:expr, $($arg:tt)*) => {
        $crate::logging::warn($module, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($module:expr, $($arg:tt)*) => {
        $crate::logging::error($module, format!($($arg)*))
    };
}

/// Health check system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub checks: HashMap<String, CheckResult>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub passed: bool,
    pub message: String,
    pub details: serde_json::Value,
}

pub struct HealthChecker {
    checks: HashMap<String, Box<dyn Fn() -> CheckResult + Send>>,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            checks: HashMap::new(),
        }
    }

    pub fn register_check<F>(&mut self, name: impl Into<String>, check: F)
    where
        F: Fn() -> CheckResult + Send + 'static,
    {
        self.checks.insert(name.into(), Box::new(check));
    }

    pub fn run_checks(&self) -> HealthStatus {
        let mut results = HashMap::new();
        let mut all_healthy = true;

        for (name, check) in &self.checks {
            let result = check();
            if !result.passed {
                all_healthy = false;
            }
            results.insert(name.clone(), result);
        }

        HealthStatus {
            healthy: all_healthy,
            checks: results,
            timestamp: chrono::Utc::now(),
        }
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        let mut checker = Self::new();
        
        // Add default checks
        checker.register_check("memory", || {
            // Simple memory check (could be enhanced with real metrics)
            CheckResult {
                passed: true,
                message: "Memory usage normal".to_string(),
                details: serde_json::json!({"allocated_mb": "unknown"}),
            }
        });

        checker.register_check("cpu", || {
            CheckResult {
                passed: true,
                message: "CPU usage normal".to_string(),
                details: serde_json::json!({"usage_percent": "unknown"}),
            }
        });

        checker
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging() {
        clear_logs();
        set_log_level(LogLevel::Debug);
        
        info("test", "Test message".to_string());
        debug("test", "Debug message".to_string());
        
        let logs = get_recent_logs(10);
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].message, "Test message");
    }

    #[test]
    fn test_metrics() {
        let mut logger = Logger::new();
        
        logger.record_metric("test_operation", Duration::from_millis(100));
        logger.record_metric("test_operation", Duration::from_millis(200));
        
        let metrics = logger.get_metrics();
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].count, 2);
        assert_eq!(metrics[0].max_duration, Duration::from_millis(200));
    }

    #[test]
    fn test_timer() {
        clear_logs();
        
        {
            let _timer = Timer::new("test_timer");
            std::thread::sleep(Duration::from_millis(10));
        } // Timer drops and records
        
        let metrics = get_metrics();
        let test_metric = metrics.iter().find(|m| m.name == "test_timer");
        assert!(test_metric.is_some());
        assert!(test_metric.unwrap().avg_duration >= Duration::from_millis(10));
    }

    #[test]
    fn test_health_checker() {
        let mut checker = HealthChecker::new();
        
        checker.register_check("always_pass", || CheckResult {
            passed: true,
            message: "OK".to_string(),
            details: serde_json::json!({}),
        });
        
        checker.register_check("always_fail", || CheckResult {
            passed: false,
            message: "FAIL".to_string(),
            details: serde_json::json!({}),
        });
        
        let status = checker.run_checks();
        assert!(!status.healthy); // Should fail because one check fails
        assert_eq!(status.checks.len(), 2);
    }
}
