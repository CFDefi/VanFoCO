//! Quantum validator for enforcing quantum mechanical constraints

use crate::ast::*;
use crate::error::{EngineError, Result};
use crate::typechecker::TypedAst;
use crate::VALIDATION_TOL;
use ndarray::{Array1, Array2};
use ndarray_linalg::{Eigh, UPLO};
use num_complex::Complex64;
use std::collections::HashMap;

/// Validated AST with quantum constraints verified
#[derive(Debug, Clone)]
pub struct ValidatedAst {
    pub typed_ast: TypedAst,
    pub validation_results: ValidationResults,
}

/// Results from quantum validation
#[derive(Debug, Clone, Default)]
pub struct ValidationResults {
    pub hermitian_operators: HashMap<String, bool>,
    pub psd_operators: HashMap<String, bool>,
    pub trace_values: HashMap<String, f64>,
}

/// Quantum validator
pub struct QuantumValidator {
    constants: HashMap<String, f64>,
    matrices: HashMap<String, Array2<Complex64>>,
}

impl QuantumValidator {
    pub fn new() -> Self {
        let mut matrices = HashMap::new();

        // Predefined Pauli matrices
        matrices.insert(
            "sigma_x".to_string(),
            Array2::from_shape_vec((2, 2), vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
            ])
            .unwrap(),
        );

        matrices.insert(
            "sigma_y".to_string(),
            Array2::from_shape_vec((2, 2), vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, -1.0),
                Complex64::new(0.0, 1.0),
                Complex64::new(0.0, 0.0),
            ])
            .unwrap(),
        );

        matrices.insert(
            "sigma_z".to_string(),
            Array2::from_shape_vec((2, 2), vec![
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(-1.0, 0.0),
            ])
            .unwrap(),
        );

        matrices.insert(
            "identity".to_string(),
            Array2::from_shape_vec((2, 2), vec![
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
            ])
            .unwrap(),
        );

        QuantumValidator {
            constants: HashMap::new(),
            matrices,
        }
    }

    /// Validate quantum constraints for the typed AST
    pub fn validate(&mut self, typed_ast: &TypedAst) -> Result<ValidatedAst> {
        let mut results = ValidationResults::default();

        // Collect constants and matrices from AST
        for stmt in &typed_ast.ast.statements {
            match stmt {
                Statement::ConstDecl { name, value } => {
                    self.constants.insert(name.clone(), *value);
                }
                Statement::MatrixDecl { name, value } => {
                    if let Ok(mat) = self.evaluate_matrix_literal(value) {
                        self.matrices.insert(name.clone(), mat);
                    }
                }
                Statement::HamiltonianDef { name, expr, .. } => {
                    if let Ok(mat) = self.evaluate_expr_to_matrix(expr) {
                        // Check Hermiticity
                        let is_hermitian = self.check_hermitian(&mat)?;
                        results.hermitian_operators.insert(name.clone(), is_hermitian);
                        
                        if !is_hermitian {
                            return Err(EngineError::quantum_error(format!(
                                "Hamiltonian '{}' is not Hermitian",
                                name
                            )));
                        }
                        
                        self.matrices.insert(name.clone(), mat);
                    }
                }
                Statement::MeasurementDef { name, spec } => {
                    self.validate_measurement(name, spec, &mut results)?;
                }
                Statement::Experiment { body, .. } => {
                    self.validate_experiment(body, &mut results)?;
                }
                _ => {}
            }
        }

        Ok(ValidatedAst {
            typed_ast: typed_ast.clone(),
            validation_results: results,
        })
    }

    /// Check if a matrix is Hermitian (A = A†)
    fn check_hermitian(&self, mat: &Array2<Complex64>) -> Result<bool> {
        let (n, m) = mat.dim();
        if n != m {
            return Ok(false);
        }

        let mut max_dev = 0.0;
        for i in 0..n {
            for j in 0..m {
                let diff = (mat[[i, j]] - mat[[j, i]].conj()).norm();
                max_dev = max_dev.max(diff);
            }
        }

        if max_dev > VALIDATION_TOL {
            Err(EngineError::NotHermitian { deviation: max_dev })
        } else {
            Ok(true)
        }
    }

    /// Check if a matrix is positive semi-definite (all eigenvalues >= 0)
    fn check_psd(&self, mat: &Array2<Complex64>) -> Result<bool> {
        let (n, m) = mat.dim();
        if n != m {
            return Err(EngineError::validation_error("PSD check requires square matrix"));
        }

        // First check if Hermitian
        if !self.check_hermitian(mat).unwrap_or(false) {
            return Err(EngineError::validation_error("PSD check requires Hermitian matrix"));
        }

        // Compute eigenvalues
        let eigenvalues = mat.eigh(UPLO::Upper)
            .map_err(|e| EngineError::validation_error(format!("Failed to compute eigenvalues: {}", e)))?;
        
        let min_eigenvalue = eigenvalues.0.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
        if min_eigenvalue < -VALIDATION_TOL {
            Err(EngineError::NotPSD { min_eigenvalue })
        } else {
            Ok(true)
        }
    }

    /// Check trace of a matrix
    fn check_trace(&self, mat: &Array2<Complex64>, expected: f64) -> Result<()> {
        let (n, m) = mat.dim();
        if n != m {
            return Err(EngineError::validation_error("Trace requires square matrix"));
        }

        let trace: Complex64 = (0..n).map(|i| mat[[i, i]]).sum();
        let trace_real = trace.re;
        
        if (trace_real - expected).abs() > VALIDATION_TOL {
            Err(EngineError::TraceError {
                expected,
                actual: trace_real,
            })
        } else {
            Ok(())
        }
    }

    fn validate_measurement(
        &self,
        name: &str,
        spec: &MeasurementSpec,
        results: &mut ValidationResults,
    ) -> Result<()> {
        match spec {
            MeasurementSpec::Projective { projectors } => {
                // Check each projector is Hermitian and P^2 = P
                for (i, proj_literal) in projectors.iter().enumerate() {
                    let proj = self.evaluate_matrix_literal(proj_literal)?;
                    
                    // Check Hermiticity
                    self.check_hermitian(&proj)?;
                    
                    // Check idempotence: P^2 = P
                    let proj_squared = proj.dot(&proj);
                    let diff = &proj_squared - &proj;
                    let norm = diff.iter().map(|x| x.norm()).fold(0.0, f64::max);
                    
                    if norm > VALIDATION_TOL {
                        return Err(EngineError::quantum_error(format!(
                            "Projector {} in measurement '{}' is not idempotent",
                            i, name
                        )));
                    }
                }
                
                // Check completeness: sum of projectors = identity
                if !projectors.is_empty() {
                    let first = self.evaluate_matrix_literal(&projectors[0])?;
                    let dim = first.nrows();
                    let mut sum = Array2::zeros((dim, dim));
                    
                    for proj_literal in projectors {
                        let proj = self.evaluate_matrix_literal(proj_literal)?;
                        sum = sum + proj;
                    }
                    
                    let identity = Array2::from_diag(&Array1::from_elem(dim, Complex64::new(1.0, 0.0)));
                    let diff = sum - identity;
                    let norm = diff.iter().map(|x| x.norm()).fold(0.0, f64::max);
                    
                    if norm > VALIDATION_TOL {
                        return Err(EngineError::quantum_error(format!(
                            "Projectors in measurement '{}' do not sum to identity",
                            name
                        )));
                    }
                }
            }
            MeasurementSpec::POVM { effects } => {
                // Check each effect is PSD and effects sum to identity
                for (i, effect_literal) in effects.iter().enumerate() {
                    let effect = self.evaluate_matrix_literal(effect_literal)?;
                    self.check_psd(&effect).map_err(|_| {
                        EngineError::quantum_error(format!(
                            "POVM effect {} in measurement '{}' is not PSD",
                            i, name
                        ))
                    })?;
                }
                
                // Check completeness
                if !effects.is_empty() {
                    let first = self.evaluate_matrix_literal(&effects[0])?;
                    let dim = first.nrows();
                    let mut sum = Array2::zeros((dim, dim));
                    
                    for effect_literal in effects {
                        let effect = self.evaluate_matrix_literal(effect_literal)?;
                        sum = sum + effect;
                    }
                    
                    let identity = Array2::from_diag(&Array1::from_elem(dim, Complex64::new(1.0, 0.0)));
                    let diff = sum - identity;
                    let norm = diff.iter().map(|x| x.norm()).fold(0.0, f64::max);
                    
                    if norm > VALIDATION_TOL {
                        return Err(EngineError::quantum_error(format!(
                            "POVM effects in measurement '{}' do not sum to identity",
                            name
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_experiment(&self, body: &ExperimentBody, results: &mut ValidationResults) -> Result<()> {
        if let Some(init) = &body.init {
            match init {
                StateSpec::Ket(vec) => {
                    // Check normalization
                    let ket = self.evaluate_vector_literal(vec)?;
                    let norm_sq: f64 = ket.iter().map(|x| x.norm_sqr()).sum();
                    
                    if (norm_sq - 1.0).abs() > VALIDATION_TOL {
                        return Err(EngineError::quantum_error(format!(
                            "Initial ket is not normalized: ||ψ||² = {}",
                            norm_sq
                        )));
                    }
                }
                StateSpec::Rho(mat) => {
                    let rho = self.evaluate_matrix_literal(mat)?;
                    
                    // Check Hermitian
                    self.check_hermitian(&rho)?;
                    
                    // Check PSD
                    self.check_psd(&rho)?;
                    
                    // Check trace = 1
                    self.check_trace(&rho, 1.0)?;
                }
            }
        }
        Ok(())
    }

    fn evaluate_matrix_literal(&self, mat: &MatrixLiteral) -> Result<Array2<Complex64>> {
        let n_rows = mat.rows.len();
        let n_cols = mat.rows[0].len();
        
        let mut data = Vec::with_capacity(n_rows * n_cols);
        
        for row in &mat.rows {
            for elem in row {
                let val = self.evaluate_expr_to_complex(elem)?;
                data.push(val);
            }
        }
        
        Array2::from_shape_vec((n_rows, n_cols), data)
            .map_err(|e| EngineError::validation_error(format!("Failed to create matrix: {}", e)))
    }

    fn evaluate_vector_literal(&self, vec: &VectorLiteral) -> Result<Array1<Complex64>> {
        let data: Result<Vec<Complex64>> = vec.elements.iter()
            .map(|e| self.evaluate_expr_to_complex(e))
            .collect();
        
        Ok(Array1::from_vec(data?))
    }

    fn evaluate_expr_to_complex(&self, expr: &Expr) -> Result<Complex64> {
        match expr {
            Expr::Number(x) => Ok(Complex64::new(*x, 0.0)),
            Expr::ComplexNumber(c) => Ok(*c),
            Expr::Identifier(name) => {
                if let Some(&val) = self.constants.get(name) {
                    Ok(Complex64::new(val, 0.0))
                } else {
                    Err(EngineError::validation_error(format!(
                        "Cannot evaluate identifier '{}' to constant",
                        name
                    )))
                }
            }
            Expr::Add(a, b) => {
                Ok(self.evaluate_expr_to_complex(a)? + self.evaluate_expr_to_complex(b)?)
            }
            Expr::Sub(a, b) => {
                Ok(self.evaluate_expr_to_complex(a)? - self.evaluate_expr_to_complex(b)?)
            }
            Expr::Mul(a, b) => {
                Ok(self.evaluate_expr_to_complex(a)? * self.evaluate_expr_to_complex(b)?)
            }
            Expr::Div(a, b) => {
                Ok(self.evaluate_expr_to_complex(a)? / self.evaluate_expr_to_complex(b)?)
            }
            _ => Err(EngineError::validation_error(
                "Expression too complex to evaluate at validation time",
            )),
        }
    }

    fn evaluate_expr_to_matrix(&self, expr: &Expr) -> Result<Array2<Complex64>> {
        match expr {
            Expr::Matrix(mat) => self.evaluate_matrix_literal(mat),
            Expr::Identifier(name) => {
                self.matrices.get(name).cloned().ok_or_else(|| {
                    EngineError::validation_error(format!("Matrix '{}' not found", name))
                })
            }
            Expr::Mul(a, b) => {
                let left = self.evaluate_expr_to_matrix(a)?;
                let right = self.evaluate_expr_to_matrix(b)?;
                Ok(left.dot(&right))
            }
            Expr::Add(a, b) => {
                let left = self.evaluate_expr_to_matrix(a)?;
                let right = self.evaluate_expr_to_matrix(b)?;
                Ok(left + right)
            }
            Expr::Sub(a, b) => {
                let left = self.evaluate_expr_to_matrix(a)?;
                let right = self.evaluate_expr_to_matrix(b)?;
                Ok(left - right)
            }
            _ => Err(EngineError::Unsupported(
                "Complex expression evaluation not yet implemented".to_string(),
            )),
        }
    }
}

impl Default for QuantumValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pauli_hermitian() {
        let validator = QuantumValidator::new();
        let sigma_x = validator.matrices.get("sigma_x").unwrap();
        assert!(validator.check_hermitian(sigma_x).is_ok());
    }

    #[test]
    fn test_identity_psd() {
        let validator = QuantumValidator::new();
        let identity = validator.matrices.get("identity").unwrap();
        assert!(validator.check_psd(identity).is_ok());
    }

    #[test]
    fn test_trace_identity() {
        let validator = QuantumValidator::new();
        let identity = validator.matrices.get("identity").unwrap();
        assert!(validator.check_trace(identity, 2.0).is_ok());
    }
}
