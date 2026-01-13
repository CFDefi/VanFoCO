//! Abstract Syntax Tree (AST) definitions for the quantum DSL

use num_complex::Complex64;
use serde::{Deserialize, Serialize};

/// Complete program AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

/// Top-level statements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    ConstDecl {
        name: String,
        value: f64,
    },
    SymbolDecl {
        name: String,
    },
    MatrixDecl {
        name: String,
        value: MatrixLiteral,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
    HamiltonianDef {
        name: String,
        params: Vec<String>,
        expr: Box<Expr>,
    },
    MeasurementDef {
        name: String,
        spec: MeasurementSpec,
    },
    Experiment {
        name: String,
        body: ExperimentBody,
    },
}

/// Expressions in the DSL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    // Literals
    Number(f64),
    ComplexNumber(Complex64),
    Identifier(String),
    Matrix(MatrixLiteral),
    Vector(VectorLiteral),

    // Binary operations
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),

    // Quantum operations
    Dagger(Box<Expr>),
    Trace(Box<Expr>),
    Tensor(Box<Expr>, Box<Expr>),
    Commutator(Box<Expr>, Box<Expr>),
    AntiCommutator(Box<Expr>, Box<Expr>),

    // Mathematical functions
    Expm(Box<Expr>),
    Sqrt(Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Exp(Box<Expr>),

    // Function application
    FuncCall {
        name: String,
        args: Vec<Expr>,
    },
}

/// Matrix literal (2D array of expressions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixLiteral {
    pub rows: Vec<Vec<Expr>>,
}

/// Vector literal (1D array of expressions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorLiteral {
    pub elements: Vec<Expr>,
}

/// Measurement specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementSpec {
    Projective { projectors: Vec<MatrixLiteral> },
    POVM { effects: Vec<MatrixLiteral> },
}

/// Experiment body containing initialization, evolution, and measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentBody {
    pub init: Option<StateSpec>,
    pub evolution: Option<EvolutionSpec>,
    pub measurements: Option<MeasurementSchedule>,
}

/// Initial state specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateSpec {
    Ket(VectorLiteral),
    Rho(MatrixLiteral),
}

/// Evolution specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionSpec {
    pub state_name: String,
    pub hamiltonian_name: String,
    pub timegrid: TimeGrid,
    pub lindblad_ops: Vec<LindbladTerm>,
}

/// Time grid specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeGrid {
    Regular {
        t0: f64,
        dt: f64,
        n_steps: usize,
    },
    Explicit {
        times: Vec<f64>,
    },
}

/// Lindblad operator term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LindbladTerm {
    pub operator_name: String,
    pub rate: Box<Expr>,
}

/// Measurement schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementSchedule {
    pub events: Vec<MeasurementEvent>,
}

/// Single measurement event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementEvent {
    pub time: f64,
    pub measurement_name: String,
}

impl Ast {
    pub fn new(statements: Vec<Statement>) -> Self {
        Ast { statements }
    }
}

impl TimeGrid {
    /// Get all time points from the grid
    pub fn get_times(&self) -> Vec<f64> {
        match self {
            TimeGrid::Regular { t0, dt, n_steps } => {
                (0..=*n_steps).map(|i| t0 + (i as f64) * dt).collect()
            }
            TimeGrid::Explicit { times } => times.clone(),
        }
    }

    /// Get number of time steps
    pub fn len(&self) -> usize {
        match self {
            TimeGrid::Regular { n_steps, .. } => *n_steps + 1,
            TimeGrid::Explicit { times } => times.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timegrid_regular() {
        let grid = TimeGrid::Regular {
            t0: 0.0,
            dt: 0.1,
            n_steps: 10,
        };
        let times = grid.get_times();
        assert_eq!(times.len(), 11);
        assert_eq!(times[0], 0.0);
        assert!((times[10] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_timegrid_explicit() {
        let grid = TimeGrid::Explicit {
            times: vec![0.0, 0.5, 1.0],
        };
        assert_eq!(grid.len(), 3);
        assert_eq!(grid.get_times(), vec![0.0, 0.5, 1.0]);
    }
}
