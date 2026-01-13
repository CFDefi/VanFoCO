//! Intermediate Representation (IR) for quantum computations
//!
//! The IR represents the computation as a DAG of kernel operations
//! that can be executed on different backends (CPU/GPU).

use num_complex::Complex64;
use ndarray::Array2;
use serde::{Deserialize, Serialize};

/// IR node ID
pub type NodeId = usize;

/// Complete IR program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrProgram {
    pub nodes: Vec<IrNode>,
    pub experiments: Vec<IrExperiment>,
}

/// Single IR node representing an operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrNode {
    /// Load a constant matrix
    LoadMatrix {
        id: NodeId,
        name: String,
        data: Vec<Complex64>,
        shape: (usize, usize),
    },

    /// Load a vector
    LoadVector {
        id: NodeId,
        name: String,
        data: Vec<Complex64>,
    },

    /// Scalar value
    Scalar {
        id: NodeId,
        value: Complex64,
    },

    /// Matrix addition
    MatrixAdd {
        id: NodeId,
        left: NodeId,
        right: NodeId,
    },

    /// Matrix multiplication
    MatrixMul {
        id: NodeId,
        left: NodeId,
        right: NodeId,
    },

    /// Scalar multiplication
    ScalarMul {
        id: NodeId,
        scalar: NodeId,
        matrix: NodeId,
    },

    /// Matrix exponential (e^A)
    MatrixExp {
        id: NodeId,
        input: NodeId,
    },

    /// Tensor product
    TensorProduct {
        id: NodeId,
        left: NodeId,
        right: NodeId,
    },

    /// Hermitian conjugate (dagger)
    Dagger {
        id: NodeId,
        input: NodeId,
    },

    /// Trace
    Trace {
        id: NodeId,
        input: NodeId,
    },

    /// Commutator [A, B] = AB - BA
    Commutator {
        id: NodeId,
        left: NodeId,
        right: NodeId,
    },

    /// Unitary evolution: U(t) = exp(-iHt)
    UnitaryPropagator {
        id: NodeId,
        hamiltonian: NodeId,
        time: f64,
    },

    /// Apply unitary to ket: |ψ'⟩ = U|ψ⟩
    ApplyUnitaryKet {
        id: NodeId,
        unitary: NodeId,
        ket: NodeId,
    },

    /// Apply unitary to density matrix: ρ' = UρU†
    ApplyUnitaryRho {
        id: NodeId,
        unitary: NodeId,
        rho: NodeId,
    },

    /// ODE integration for Lindblad master equation
    IntegrateLindblad {
        id: NodeId,
        hamiltonian: NodeId,
        initial_rho: NodeId,
        lindblad_ops: Vec<LindbladOperator>,
        times: Vec<f64>,
    },

    /// Measure observable expectation: ⟨O⟩ = Tr(Oρ)
    MeasureExpectation {
        id: NodeId,
        observable: NodeId,
        state: NodeId,
    },

    /// Projective measurement probabilities
    MeasureProjective {
        id: NodeId,
        projectors: Vec<NodeId>,
        state: NodeId,
    },
}

/// Lindblad operator with rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LindbladOperator {
    pub operator: NodeId,
    pub rate: f64,
}

/// Experiment specification in IR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrExperiment {
    pub name: String,
    pub initial_state: NodeId,
    pub evolution: Option<IrEvolution>,
    pub measurements: Vec<IrMeasurement>,
}

/// Evolution specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrEvolution {
    pub method: EvolutionMethod,
    pub times: Vec<f64>,
}

/// Evolution method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionMethod {
    Schrodinger { hamiltonian: NodeId },
    Lindblad { hamiltonian: NodeId, operators: Vec<LindbladOperator> },
}

/// Measurement specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrMeasurement {
    pub time_index: usize,
    pub measurement_type: MeasurementType,
}

/// Measurement type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementType {
    Projective { projectors: Vec<NodeId> },
    POVM { effects: Vec<NodeId> },
    Observable { operator: NodeId },
}

impl IrProgram {
    pub fn new() -> Self {
        IrProgram {
            nodes: Vec::new(),
            experiments: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: IrNode) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }
}

impl Default for IrProgram {
    fn default() -> Self {
        Self::new()
    }
}

impl IrNode {
    pub fn id(&self) -> NodeId {
        match self {
            IrNode::LoadMatrix { id, .. } => *id,
            IrNode::LoadVector { id, .. } => *id,
            IrNode::Scalar { id, .. } => *id,
            IrNode::MatrixAdd { id, .. } => *id,
            IrNode::MatrixMul { id, .. } => *id,
            IrNode::ScalarMul { id, .. } => *id,
            IrNode::MatrixExp { id, .. } => *id,
            IrNode::TensorProduct { id, .. } => *id,
            IrNode::Dagger { id, .. } => *id,
            IrNode::Trace { id, .. } => *id,
            IrNode::Commutator { id, .. } => *id,
            IrNode::UnitaryPropagator { id, .. } => *id,
            IrNode::ApplyUnitaryKet { id, .. } => *id,
            IrNode::ApplyUnitaryRho { id, .. } => *id,
            IrNode::IntegrateLindblad { id, .. } => *id,
            IrNode::MeasureExpectation { id, .. } => *id,
            IrNode::MeasureProjective { id, .. } => *id,
        }
    }
}
