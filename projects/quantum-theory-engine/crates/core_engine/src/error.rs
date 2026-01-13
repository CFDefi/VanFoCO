//! Error types for the quantum theory engine

use thiserror::Error;

/// Result type alias for engine operations
pub type Result<T> = std::result::Result<T, EngineError>;

/// Main error type for the quantum theory engine
#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Quantum constraint violation: {0}")]
    QuantumConstraintError(String),

    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: String, actual: String },

    #[error("Matrix is not Hermitian: max deviation {deviation:.2e}")]
    NotHermitian { deviation: f64 },

    #[error("Matrix is not positive semi-definite: minimum eigenvalue {min_eigenvalue:.2e}")]
    NotPSD { min_eigenvalue: f64 },

    #[error("Trace constraint violation: expected {expected:.6}, got {actual:.6}")]
    TraceError { expected: f64, actual: f64 },

    #[error("Channel is not CPTP: {reason}")]
    NotCPTP { reason: String },

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("ODE integration failed: {0}")]
    IntegrationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl EngineError {
    /// Create a parse error with location information
    pub fn parse_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        EngineError::ParseError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a type error
    pub fn type_error(message: impl Into<String>) -> Self {
        EngineError::TypeError(message.into())
    }

    /// Create a validation error
    pub fn validation_error(message: impl Into<String>) -> Self {
        EngineError::ValidationError(message.into())
    }

    /// Create a quantum constraint error
    pub fn quantum_error(message: impl Into<String>) -> Self {
        EngineError::QuantumConstraintError(message.into())
    }

    /// Create a dimension mismatch error
    pub fn dimension_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        EngineError::DimensionMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }
}
