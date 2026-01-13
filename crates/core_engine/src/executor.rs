//! Executor for running IR programs on different backends

use crate::error::{EngineError, Result};
use crate::ir::*;
use crate::kernels_cpu;
use crate::ode::{Rk4Integrator, evolve_unitary};
use ndarray::{Array1, Array2};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub backend_type: BackendType,
    pub num_threads: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackendType {
    CpuDense,
    CpuSparse,
    Gpu,
}

impl Default for BackendConfig {
    fn default() -> Self {
        BackendConfig {
            backend_type: BackendType::CpuDense,
            num_threads: None,
        }
    }
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub experiment_results: Vec<ExperimentResult>,
}

/// Result for a single experiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub name: String,
    pub times: Vec<f64>,
    pub state_type: StateType,
    pub measurements: Vec<MeasurementResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateType {
    PureState,
    DensityMatrix,
}

/// Measurement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementResult {
    pub time: f64,
    pub probabilities: Vec<f64>,
}

/// Executor for IR programs
pub struct Executor {
    config: BackendConfig,
    matrix_cache: HashMap<NodeId, Array2<Complex64>>,
    vector_cache: HashMap<NodeId, Array1<Complex64>>,
    scalar_cache: HashMap<NodeId, Complex64>,
}

impl Executor {
    pub fn new(config: BackendConfig) -> Self {
        Executor {
            config,
            matrix_cache: HashMap::new(),
            vector_cache: HashMap::new(),
            scalar_cache: HashMap::new(),
        }
    }

    /// Execute an IR program
    pub fn execute(&mut self, ir: &IrProgram) -> Result<ExecutionResult> {
        // Load all nodes into cache
        for node in &ir.nodes {
            self.load_node(node)?;
        }

        // Execute experiments
        let mut experiment_results = Vec::new();
        for experiment in &ir.experiments {
            let result = self.execute_experiment(experiment)?;
            experiment_results.push(result);
        }

        Ok(ExecutionResult {
            experiment_results,
        })
    }

    fn load_node(&mut self, node: &IrNode) -> Result<()> {
        match node {
            IrNode::LoadMatrix { id, data, shape, .. } => {
                let mat = Array2::from_shape_vec(*shape, data.clone())
                    .map_err(|e| EngineError::ExecutionError(format!("Failed to load matrix: {}", e)))?;
                self.matrix_cache.insert(*id, mat);
            }
            IrNode::LoadVector { id, data, .. } => {
                let vec = Array1::from_vec(data.clone());
                self.vector_cache.insert(*id, vec);
            }
            IrNode::Scalar { id, value } => {
                self.scalar_cache.insert(*id, *value);
            }
            _ => {} // Other nodes computed on demand
        }
        Ok(())
    }

    fn execute_experiment(&mut self, experiment: &IrExperiment) -> Result<ExperimentResult> {
        // Get initial state
        let initial_state_id = experiment.initial_state;
        let (times, state_type) = if let Some(evolution) = &experiment.evolution {
            let times = evolution.times.clone();
            let state_type = if self.vector_cache.contains_key(&initial_state_id) {
                StateType::PureState
            } else {
                StateType::DensityMatrix
            };
            (times, state_type)
        } else {
            (vec![0.0], StateType::DensityMatrix)
        };

        // Execute evolution
        let final_states = if let Some(evolution) = &experiment.evolution {
            self.execute_evolution(initial_state_id, evolution)?
        } else {
            // No evolution, just initial state
            if let Some(ket) = self.vector_cache.get(&initial_state_id) {
                vec![kernels_cpu::ket_to_rho(ket)]
            } else if let Some(rho) = self.matrix_cache.get(&initial_state_id) {
                vec![rho.clone()]
            } else {
                return Err(EngineError::ExecutionError(
                    "Initial state not found".to_string(),
                ));
            }
        };

        // Execute measurements (stub for now)
        let measurements = Vec::new();

        Ok(ExperimentResult {
            name: experiment.name.clone(),
            times,
            state_type,
            measurements,
        })
    }

    fn execute_evolution(
        &mut self,
        initial_state_id: NodeId,
        evolution: &IrEvolution,
    ) -> Result<Vec<Array2<Complex64>>> {
        match &evolution.method {
            EvolutionMethod::Schrodinger { hamiltonian } => {
                let h = self.get_matrix(*hamiltonian)?;

                if let Some(ket) = self.vector_cache.get(&initial_state_id).cloned() {
                    // Pure state evolution
                    let kets = evolve_unitary(&h, &ket, &evolution.times)?;
                    Ok(kets.iter().map(|k| kernels_cpu::ket_to_rho(k)).collect())
                } else {
                    Err(EngineError::ExecutionError(
                        "Schrödinger evolution requires initial ket".to_string(),
                    ))
                }
            }
            EvolutionMethod::Lindblad {
                hamiltonian,
                operators,
            } => {
                let h = self.get_matrix(*hamiltonian)?;
                let rho0 = if let Some(rho) = self.matrix_cache.get(&initial_state_id).cloned() {
                    rho
                } else if let Some(ket) = self.vector_cache.get(&initial_state_id) {
                    kernels_cpu::ket_to_rho(ket)
                } else {
                    return Err(EngineError::ExecutionError(
                        "Initial state not found".to_string(),
                    ));
                };

                // Get Lindblad operators
                let mut lindblad_ops = Vec::new();
                for op in operators {
                    let l = self.get_matrix(op.operator)?;
                    lindblad_ops.push((l, op.rate));
                }

                let integrator = Rk4Integrator::new(h, lindblad_ops);
                let result = integrator.integrate(rho0, &evolution.times)?;

                Ok(result.states)
            }
        }
    }

    fn get_matrix(&self, id: NodeId) -> Result<Array2<Complex64>> {
        self.matrix_cache
            .get(&id)
            .cloned()
            .ok_or_else(|| EngineError::ExecutionError(format!("Matrix {} not found", id)))
    }

    fn get_scalar(&self, id: NodeId) -> Result<Complex64> {
        self.scalar_cache
            .get(&id)
            .copied()
            .ok_or_else(|| EngineError::ExecutionError(format!("Scalar {} not found", id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let config = BackendConfig::default();
        let executor = Executor::new(config);
        assert!(executor.matrix_cache.is_empty());
    }
}
