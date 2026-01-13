//! AST to IR lowering pass

use crate::ast::*;
use crate::error::{EngineError, Result};
use crate::ir::*;
use crate::validator::ValidatedAst;
use num_complex::Complex64;
use std::collections::HashMap;

/// Lowerer converts validated AST to IR
pub struct Lowerer {
    ir: IrProgram,
    node_map: HashMap<String, NodeId>,
    next_id: NodeId,
}

impl Lowerer {
    pub fn new() -> Self {
        Lowerer {
            ir: IrProgram::new(),
            node_map: HashMap::new(),
            next_id: 0,
        }
    }

    /// Lower validated AST to IR
    pub fn lower(&mut self, validated: &ValidatedAst) -> Result<IrProgram> {
        // Process statements
        for stmt in &validated.typed_ast.ast.statements {
            self.lower_statement(stmt)?;
        }

        Ok(self.ir.clone())
    }

    fn lower_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::ConstDecl { name, value } => {
                let id = self.allocate_id();
                let node = IrNode::Scalar {
                    id,
                    value: Complex64::new(*value, 0.0),
                };
                self.ir.nodes.push(node);
                self.node_map.insert(name.clone(), id);
                Ok(())
            }
            Statement::MatrixDecl { name, value } => {
                let id = self.lower_matrix_literal(name, value)?;
                self.node_map.insert(name.clone(), id);
                Ok(())
            }
            Statement::HamiltonianDef { name, expr, .. } => {
                let id = self.lower_expr(expr)?;
                self.node_map.insert(name.clone(), id);
                Ok(())
            }
            Statement::Experiment { name, body } => {
                let experiment = self.lower_experiment(name, body)?;
                self.ir.experiments.push(experiment);
                Ok(())
            }
            _ => Ok(()), // Other statements handled in earlier passes
        }
    }

    fn lower_expr(&mut self, expr: &Expr) -> Result<NodeId> {
        match expr {
            Expr::Number(x) => {
                let id = self.allocate_id();
                self.ir.nodes.push(IrNode::Scalar {
                    id,
                    value: Complex64::new(*x, 0.0),
                });
                Ok(id)
            }
            Expr::Identifier(name) => {
                self.node_map.get(name).copied().ok_or_else(|| {
                    EngineError::Internal(format!("Undefined identifier in IR lowering: {}", name))
                })
            }
            Expr::Add(left, right) => {
                let left_id = self.lower_expr(left)?;
                let right_id = self.lower_expr(right)?;
                let id = self.allocate_id();
                self.ir.nodes.push(IrNode::MatrixAdd {
                    id,
                    left: left_id,
                    right: right_id,
                });
                Ok(id)
            }
            Expr::Mul(left, right) => {
                let left_id = self.lower_expr(left)?;
                let right_id = self.lower_expr(right)?;
                let id = self.allocate_id();
                self.ir.nodes.push(IrNode::MatrixMul {
                    id,
                    left: left_id,
                    right: right_id,
                });
                Ok(id)
            }
            Expr::Dagger(inner) => {
                let inner_id = self.lower_expr(inner)?;
                let id = self.allocate_id();
                self.ir.nodes.push(IrNode::Dagger {
                    id,
                    input: inner_id,
                });
                Ok(id)
            }
            Expr::Tensor(left, right) => {
                let left_id = self.lower_expr(left)?;
                let right_id = self.lower_expr(right)?;
                let id = self.allocate_id();
                self.ir.nodes.push(IrNode::TensorProduct {
                    id,
                    left: left_id,
                    right: right_id,
                });
                Ok(id)
            }
            Expr::Expm(inner) => {
                let inner_id = self.lower_expr(inner)?;
                let id = self.allocate_id();
                self.ir.nodes.push(IrNode::MatrixExp {
                    id,
                    input: inner_id,
                });
                Ok(id)
            }
            _ => Err(EngineError::Unsupported(format!(
                "Expression lowering not yet implemented: {:?}",
                expr
            ))),
        }
    }

    fn lower_matrix_literal(&mut self, name: &str, mat: &MatrixLiteral) -> Result<NodeId> {
        let n_rows = mat.rows.len();
        let n_cols = mat.rows[0].len();
        let mut data = Vec::with_capacity(n_rows * n_cols);

        for row in &mat.rows {
            for elem in row {
                let val = self.evaluate_to_complex(elem)?;
                data.push(val);
            }
        }

        let id = self.allocate_id();
        self.ir.nodes.push(IrNode::LoadMatrix {
            id,
            name: name.to_string(),
            data,
            shape: (n_rows, n_cols),
        });

        Ok(id)
    }

    fn lower_experiment(&mut self, name: &str, body: &ExperimentBody) -> Result<IrExperiment> {
        // Lower initial state
        let initial_state = if let Some(init) = &body.init {
            match init {
                StateSpec::Ket(vec) => self.lower_vector_literal("init_ket", vec)?,
                StateSpec::Rho(mat) => self.lower_matrix_literal("init_rho", mat)?,
            }
        } else {
            return Err(EngineError::validation_error(
                "Experiment must have initial state",
            ));
        };

        // Lower evolution
        let evolution = if let Some(evol) = &body.evolution {
            Some(self.lower_evolution(evol)?)
        } else {
            None
        };

        // Lower measurements (stub for now)
        let measurements = Vec::new();

        Ok(IrExperiment {
            name: name.to_string(),
            initial_state,
            evolution,
            measurements,
        })
    }

    fn lower_evolution(&mut self, evol: &EvolutionSpec) -> Result<IrEvolution> {
        let hamiltonian_id = self.node_map.get(&evol.hamiltonian_name).copied().ok_or_else(|| {
            EngineError::Internal(format!("Hamiltonian '{}' not found", evol.hamiltonian_name))
        })?;

        let times = evol.timegrid.get_times();

        let method = if evol.lindblad_ops.is_empty() {
            EvolutionMethod::Schrodinger {
                hamiltonian: hamiltonian_id,
            }
        } else {
            // Lower Lindblad operators
            let mut operators = Vec::new();
            for term in &evol.lindblad_ops {
                let op_id = self.node_map.get(&term.operator_name).copied().ok_or_else(|| {
                    EngineError::Internal(format!("Lindblad operator '{}' not found", term.operator_name))
                })?;
                
                // Evaluate rate (assume constant for now)
                let rate = if let Expr::Number(r) = &*term.rate {
                    *r
                } else {
                    return Err(EngineError::Unsupported(
                        "Non-constant Lindblad rates not yet supported".to_string(),
                    ));
                };

                operators.push(LindbladOperator {
                    operator: op_id,
                    rate,
                });
            }

            EvolutionMethod::Lindblad {
                hamiltonian: hamiltonian_id,
                operators,
            }
        };

        Ok(IrEvolution { method, times })
    }

    fn lower_vector_literal(&mut self, name: &str, vec: &VectorLiteral) -> Result<NodeId> {
        let mut data = Vec::with_capacity(vec.elements.len());

        for elem in &vec.elements {
            let val = self.evaluate_to_complex(elem)?;
            data.push(val);
        }

        let id = self.allocate_id();
        self.ir.nodes.push(IrNode::LoadVector {
            id,
            name: name.to_string(),
            data,
        });

        Ok(id)
    }

    fn evaluate_to_complex(&self, expr: &Expr) -> Result<Complex64> {
        match expr {
            Expr::Number(x) => Ok(Complex64::new(*x, 0.0)),
            Expr::ComplexNumber(c) => Ok(*c),
            _ => Err(EngineError::Unsupported(
                "Complex expression evaluation in lowering not yet implemented".to_string(),
            )),
        }
    }

    fn allocate_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl Default for Lowerer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowerer_creation() {
        let lowerer = Lowerer::new();
        assert_eq!(lowerer.next_id, 0);
    }
}
