//! Quantum Theory Engine - Core Library
//!
//! An industrial-grade quantum simulation framework for testing quantum theories
//! against experimental data.
//!
//! # Architecture
//!
//! The engine follows a multi-stage compilation pipeline:
//! 1. **Parser**: DSL text → AST
//! 2. **Type Checker**: Shape and dimension validation
//! 3. **Quantum Validator**: Domain-specific constraints (Hermiticity, PSD, CPTP)
//! 4. **Optimizer**: CSE, constant folding, sparsity detection
//! 5. **Lowering**: AST → IR (kernel calls)
//! 6. **Executor**: Backend-specific execution (CPU/GPU)
//!
//! # Example
//!
//! ```rust,ignore
//! use quantum_theory_engine::{parse_dsl, execute, BackendConfig};
//!
//! let source = r#"
//!     experiment rabi {
//!         const omega = 1.0;
//!         Hamiltonian H = (omega/2) * sigma_z;
//!         init: ket(vec(1, 0));
//!         evolution: evolve(init, H, timegrid=(0.0, 0.01, 101));
//!     }
//! "#;
//!
//! let ast = parse_dsl(source)?;
//! let validated = validate_quantum(&ast)?;
//! let result = execute(&validated, BackendConfig::default())?;
//! ```

pub mod ast;
pub mod error;
pub mod executor;
pub mod ir;
pub mod job_queue;
pub mod kernels_cpu;
pub mod logging;
pub mod lowering;
pub mod ode;
pub mod optimizer;
pub mod parser;
pub mod prover;
pub mod stats;
pub mod streaming;
pub mod templates;
pub mod typechecker;
pub mod validator;

// Re-exports for convenience
pub use ast::{Ast, Expr, Statement};
pub use job_queue::{JobQueue, Job, JobKind, Priority, JobStatus};
pub use logging::{LogLevel, Timer, HealthChecker, HealthStatus};
pub use prover::{Prover, ProofResult, Property, PropertyProof};
pub use streaming::{StreamingManager, RollingFitEngine, DataPoint};
pub use templates::{TemplateRegistry, Template};
pub use error::{EngineError, Result};
pub use executor::{BackendConfig, ExecutionResult, Executor};
pub use parser::parse_dsl;
pub use prover::{Prover, ProofResult, Property, PropertyProof};
pub use stats::{FitResult, TestResult};
pub use typechecker::TypeChecker;
pub use validator::QuantumValidator;

/// Version of the quantum theory engine
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum Hilbert space dimension supported in MVP
pub const MAX_HILBERT_DIM: usize = 4;

/// Numerical tolerance for validation checks
pub const VALIDATION_TOL: f64 = 1e-10;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
