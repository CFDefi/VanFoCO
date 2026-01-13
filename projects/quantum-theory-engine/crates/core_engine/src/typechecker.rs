//! Type checker for the quantum DSL
//!
//! Validates shapes and dimensions of matrices and vectors, ensuring
//! mathematical operations are well-defined.

use crate::ast::*;
use crate::error::{EngineError, Result};
use std::collections::HashMap;

/// Shape information for expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Scalar,
    Vector(usize),
    Matrix(usize, usize),
}

/// Type-checked AST with shape annotations
#[derive(Debug, Clone)]
pub struct TypedAst {
    pub ast: Ast,
    pub shapes: HashMap<String, Shape>,
}

/// Type checker for quantum DSL
pub struct TypeChecker {
    shapes: HashMap<String, Shape>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut shapes = HashMap::new();

        // Predefined Pauli matrices (2x2)
        shapes.insert("sigma_x".to_string(), Shape::Matrix(2, 2));
        shapes.insert("sigma_y".to_string(), Shape::Matrix(2, 2));
        shapes.insert("sigma_z".to_string(), Shape::Matrix(2, 2));
        shapes.insert("identity".to_string(), Shape::Matrix(2, 2));

        TypeChecker { shapes }
    }

    /// Type check the entire AST
    pub fn check(&mut self, ast: &Ast) -> Result<TypedAst> {
        for stmt in &ast.statements {
            self.check_statement(stmt)?;
        }

        Ok(TypedAst {
            ast: ast.clone(),
            shapes: self.shapes.clone(),
        })
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::ConstDecl { name, .. } => {
                self.shapes.insert(name.clone(), Shape::Scalar);
                Ok(())
            }
            Statement::SymbolDecl { name } => {
                self.shapes.insert(name.clone(), Shape::Scalar);
                Ok(())
            }
            Statement::MatrixDecl { name, value } => {
                let shape = self.infer_matrix_shape(value)?;
                self.shapes.insert(name.clone(), shape);
                Ok(())
            }
            Statement::FunctionDef { name, params, body } => {
                // For now, assume functions return scalars
                // More sophisticated type inference could be added
                self.shapes.insert(name.clone(), Shape::Scalar);
                self.infer_expr_shape(body)?;
                Ok(())
            }
            Statement::HamiltonianDef { name, expr, .. } => {
                let shape = self.infer_expr_shape(expr)?;
                match shape {
                    Shape::Matrix(n, m) if n == m => {
                        self.shapes.insert(name.clone(), shape);
                        Ok(())
                    }
                    _ => Err(EngineError::type_error(format!(
                        "Hamiltonian '{}' must be a square matrix, got {:?}",
                        name, shape
                    ))),
                }
            }
            Statement::MeasurementDef { name, spec } => {
                self.check_measurement_spec(spec)?;
                Ok(())
            }
            Statement::Experiment { body, .. } => self.check_experiment_body(body),
        }
    }

    fn infer_matrix_shape(&self, mat: &MatrixLiteral) -> Result<Shape> {
        if mat.rows.is_empty() {
            return Err(EngineError::type_error("Empty matrix"));
        }

        let n_rows = mat.rows.len();
        let n_cols = mat.rows[0].len();

        // Check all rows have same length
        for (i, row) in mat.rows.iter().enumerate() {
            if row.len() != n_cols {
                return Err(EngineError::type_error(format!(
                    "Inconsistent row length in matrix: row 0 has {} elements, row {} has {}",
                    n_cols,
                    i,
                    row.len()
                )));
            }
        }

        Ok(Shape::Matrix(n_rows, n_cols))
    }

    fn infer_expr_shape(&self, expr: &Expr) -> Result<Shape> {
        match expr {
            Expr::Number(_) | Expr::ComplexNumber(_) => Ok(Shape::Scalar),
            Expr::Identifier(name) => self
                .shapes
                .get(name)
                .cloned()
                .ok_or_else(|| EngineError::type_error(format!("Unknown identifier: {}", name))),
            Expr::Matrix(mat) => self.infer_matrix_shape(mat),
            Expr::Vector(vec) => Ok(Shape::Vector(vec.elements.len())),
            Expr::Add(left, right) | Expr::Sub(left, right) => {
                let left_shape = self.infer_expr_shape(left)?;
                let right_shape = self.infer_expr_shape(right)?;
                if left_shape != right_shape {
                    return Err(EngineError::dimension_mismatch(
                        format!("{:?}", left_shape),
                        format!("{:?}", right_shape),
                    ));
                }
                Ok(left_shape)
            }
            Expr::Mul(left, right) => {
                let left_shape = self.infer_expr_shape(left)?;
                let right_shape = self.infer_expr_shape(right)?;

                match (left_shape, right_shape) {
                    (Shape::Scalar, s) | (s, Shape::Scalar) => Ok(s),
                    (Shape::Matrix(n1, m1), Shape::Matrix(n2, m2)) => {
                        if m1 != n2 {
                            return Err(EngineError::dimension_mismatch(
                                format!("{}x{}", n1, m1),
                                format!("{}x{}", n2, m2),
                            ));
                        }
                        Ok(Shape::Matrix(n1, m2))
                    }
                    (Shape::Matrix(n, m), Shape::Vector(v)) => {
                        if m != v {
                            return Err(EngineError::dimension_mismatch(
                                format!("{}x{} matrix", n, m),
                                format!("vector of length {}", v),
                            ));
                        }
                        Ok(Shape::Vector(n))
                    }
                    _ => Err(EngineError::type_error(
                        "Invalid multiplication of incompatible types",
                    )),
                }
            }
            Expr::Div(left, right) => {
                let left_shape = self.infer_expr_shape(left)?;
                let right_shape = self.infer_expr_shape(right)?;
                match right_shape {
                    Shape::Scalar => Ok(left_shape),
                    _ => Err(EngineError::type_error("Can only divide by scalar")),
                }
            }
            Expr::Pow(base, _exp) => self.infer_expr_shape(base),
            Expr::Dagger(inner) => {
                let shape = self.infer_expr_shape(inner)?;
                match shape {
                    Shape::Matrix(n, m) => Ok(Shape::Matrix(m, n)),
                    Shape::Vector(n) => Ok(Shape::Vector(n)),
                    Shape::Scalar => Ok(Shape::Scalar),
                }
            }
            Expr::Trace(inner) => {
                let shape = self.infer_expr_shape(inner)?;
                match shape {
                    Shape::Matrix(n, m) if n == m => Ok(Shape::Scalar),
                    _ => Err(EngineError::type_error(
                        "Trace requires a square matrix",
                    )),
                }
            }
            Expr::Tensor(left, right) => {
                let left_shape = self.infer_expr_shape(left)?;
                let right_shape = self.infer_expr_shape(right)?;

                match (left_shape, right_shape) {
                    (Shape::Matrix(n1, m1), Shape::Matrix(n2, m2)) => {
                        Ok(Shape::Matrix(n1 * n2, m1 * m2))
                    }
                    (Shape::Vector(n1), Shape::Vector(n2)) => Ok(Shape::Vector(n1 * n2)),
                    _ => Err(EngineError::type_error(
                        "Tensor product requires compatible types",
                    )),
                }
            }
            Expr::Commutator(left, right) | Expr::AntiCommutator(left, right) => {
                let left_shape = self.infer_expr_shape(left)?;
                let right_shape = self.infer_expr_shape(right)?;

                match (left_shape, right_shape) {
                    (Shape::Matrix(n1, m1), Shape::Matrix(n2, m2)) if n1 == m1 && n2 == m2 && n1 == n2 => {
                        Ok(Shape::Matrix(n1, m1))
                    }
                    _ => Err(EngineError::type_error(
                        "Commutator requires square matrices of same dimension",
                    )),
                }
            }
            Expr::Expm(inner) | Expr::Sqrt(inner) => {
                let shape = self.infer_expr_shape(inner)?;
                match shape {
                    Shape::Matrix(n, m) if n == m => Ok(shape),
                    Shape::Scalar => Ok(Shape::Scalar),
                    _ => Err(EngineError::type_error("Matrix function requires square matrix or scalar")),
                }
            }
            Expr::Sin(inner) | Expr::Cos(inner) | Expr::Exp(inner) => {
                self.infer_expr_shape(inner)?;
                Ok(Shape::Scalar)
            }
            Expr::FuncCall { .. } => Ok(Shape::Scalar), // Simplified
        }
    }

    fn check_measurement_spec(&self, spec: &MeasurementSpec) -> Result<()> {
        match spec {
            MeasurementSpec::Projective { projectors } => {
                if projectors.is_empty() {
                    return Err(EngineError::validation_error("Empty projector set"));
                }

                let first_shape = self.infer_matrix_shape(&projectors[0])?;
                for proj in projectors.iter().skip(1) {
                    let shape = self.infer_matrix_shape(proj)?;
                    if shape != first_shape {
                        return Err(EngineError::dimension_mismatch(
                            format!("{:?}", first_shape),
                            format!("{:?}", shape),
                        ));
                    }
                }
                Ok(())
            }
            MeasurementSpec::POVM { effects } => {
                if effects.is_empty() {
                    return Err(EngineError::validation_error("Empty POVM effect set"));
                }

                let first_shape = self.infer_matrix_shape(&effects[0])?;
                for effect in effects.iter().skip(1) {
                    let shape = self.infer_matrix_shape(effect)?;
                    if shape != first_shape {
                        return Err(EngineError::dimension_mismatch(
                            format!("{:?}", first_shape),
                            format!("{:?}", shape),
                        ));
                    }
                }
                Ok(())
            }
        }
    }

    fn check_experiment_body(&self, body: &ExperimentBody) -> Result<()> {
        if let Some(init) = &body.init {
            match init {
                StateSpec::Ket(vec) => {
                    // Just verify it's a valid vector
                    let _ = Shape::Vector(vec.elements.len());
                }
                StateSpec::Rho(mat) => {
                    let shape = self.infer_matrix_shape(mat)?;
                    match shape {
                        Shape::Matrix(n, m) if n == m => {}
                        _ => {
                            return Err(EngineError::type_error(
                                "Initial density matrix must be square",
                            ))
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_shape_inference() {
        let checker = TypeChecker::new();
        let mat = MatrixLiteral {
            rows: vec![
                vec![Expr::Number(1.0), Expr::Number(0.0)],
                vec![Expr::Number(0.0), Expr::Number(1.0)],
            ],
        };
        let shape = checker.infer_matrix_shape(&mat).unwrap();
        assert_eq!(shape, Shape::Matrix(2, 2));
    }

    #[test]
    fn test_inconsistent_matrix_error() {
        let checker = TypeChecker::new();
        let mat = MatrixLiteral {
            rows: vec![
                vec![Expr::Number(1.0), Expr::Number(0.0)],
                vec![Expr::Number(0.0)], // Wrong length
            ],
        };
        assert!(checker.infer_matrix_shape(&mat).is_err());
    }

    #[test]
    fn test_matrix_multiply() {
        let checker = TypeChecker::new();
        let left = Expr::Matrix(MatrixLiteral {
            rows: vec![vec![Expr::Number(1.0), Expr::Number(2.0)]],
        });
        let right = Expr::Matrix(MatrixLiteral {
            rows: vec![
                vec![Expr::Number(1.0)],
                vec![Expr::Number(2.0)],
            ],
        });
        let expr = Expr::Mul(Box::new(left), Box::new(right));
        let shape = checker.infer_expr_shape(&expr).unwrap();
        assert_eq!(shape, Shape::Matrix(1, 1));
    }
}
