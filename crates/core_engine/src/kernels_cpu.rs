//! CPU kernels for linear algebra operations

use crate::error::{EngineError, Result};
use ndarray::{Array1, Array2};
use ndarray_linalg::{Eigh, Inverse, UPLO};
use num_complex::Complex64;

/// Matrix exponential via eigendecomposition
pub fn matrix_exp(mat: &Array2<Complex64>) -> Result<Array2<Complex64>> {
    let (n, m) = mat.dim();
    if n != m {
        return Err(EngineError::dimension_mismatch(
            format!("{}x{}", n, n),
            format!("{}x{}", n, m),
        ));
    }

    // For small matrices (2x2, 4x4), use direct eigendecomposition
    let (eigenvalues, eigenvectors) = mat
        .eigh(UPLO::Upper)
        .map_err(|e| EngineError::ExecutionError(format!("Eigendecomposition failed: {}", e)))?;

    // Compute exp(eigenvalues)
    let exp_eigenvalues = eigenvalues.mapv(|lambda| Complex64::new(lambda.exp(), 0.0));

    // Reconstruct: V * diag(exp(λ)) * V†
    let n = eigenvalues.len();
    let mut result = Array2::zeros((n, n));

    for i in 0..n {
        for j in 0..n {
            let mut sum = Complex64::new(0.0, 0.0);
            for k in 0..n {
                sum += eigenvectors[[i, k]] * exp_eigenvalues[k] * eigenvectors[[j, k]].conj();
            }
            result[[i, j]] = sum;
        }
    }

    Ok(result)
}

/// Tensor product of two matrices
pub fn tensor_product(
    a: &Array2<Complex64>,
    b: &Array2<Complex64>,
) -> Result<Array2<Complex64>> {
    let (na, ma) = a.dim();
    let (nb, mb) = b.dim();

    let mut result = Array2::zeros((na * nb, ma * mb));

    for i in 0..na {
        for j in 0..ma {
            for k in 0..nb {
                for l in 0..mb {
                    result[[i * nb + k, j * mb + l]] = a[[i, j]] * b[[k, l]];
                }
            }
        }
    }

    Ok(result)
}

/// Hermitian conjugate (dagger)
pub fn dagger(mat: &Array2<Complex64>) -> Array2<Complex64> {
    mat.t().mapv(|x| x.conj())
}

/// Trace of a matrix
pub fn trace(mat: &Array2<Complex64>) -> Result<Complex64> {
    let (n, m) = mat.dim();
    if n != m {
        return Err(EngineError::dimension_mismatch(
            format!("{}x{}", n, n),
            format!("{}x{}", n, m),
        ));
    }

    Ok((0..n).map(|i| mat[[i, i]]).sum())
}

/// Commutator: [A, B] = AB - BA
pub fn commutator(
    a: &Array2<Complex64>,
    b: &Array2<Complex64>,
) -> Result<Array2<Complex64>> {
    let ab = a.dot(b);
    let ba = b.dot(a);
    Ok(ab - ba)
}

/// Apply unitary to ket: |ψ'⟩ = U|ψ⟩
pub fn apply_unitary_ket(
    unitary: &Array2<Complex64>,
    ket: &Array1<Complex64>,
) -> Result<Array1<Complex64>> {
    Ok(unitary.dot(ket))
}

/// Apply unitary to density matrix: ρ' = UρU†
pub fn apply_unitary_rho(
    unitary: &Array2<Complex64>,
    rho: &Array2<Complex64>,
) -> Result<Array2<Complex64>> {
    let u_dagger = dagger(unitary);
    let temp = unitary.dot(rho);
    Ok(temp.dot(&u_dagger))
}

/// Compute expectation value: ⟨O⟩ = Tr(Oρ)
pub fn expectation(
    observable: &Array2<Complex64>,
    rho: &Array2<Complex64>,
) -> Result<Complex64> {
    let product = observable.dot(rho);
    trace(&product)
}

/// Compute measurement probabilities for projective measurement
pub fn measure_projective(
    projectors: &[Array2<Complex64>],
    rho: &Array2<Complex64>,
) -> Result<Vec<f64>> {
    let mut probabilities = Vec::with_capacity(projectors.len());

    for proj in projectors {
        let prob = expectation(proj, rho)?.re;
        probabilities.push(prob.max(0.0)); // Ensure non-negative
    }

    // Normalize to handle numerical errors
    let sum: f64 = probabilities.iter().sum();
    if sum > 0.0 {
        for p in &mut probabilities {
            *p /= sum;
        }
    }

    Ok(probabilities)
}

/// Convert ket to density matrix: ρ = |ψ⟩⟨ψ|
pub fn ket_to_rho(ket: &Array1<Complex64>) -> Array2<Complex64> {
    let n = ket.len();
    let mut rho = Array2::zeros((n, n));

    for i in 0..n {
        for j in 0..n {
            rho[[i, j]] = ket[i] * ket[j].conj();
        }
    }

    rho
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_dagger() {
        let mat = Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 1.0),
                Complex64::new(0.0, -1.0),
                Complex64::new(1.0, 0.0),
            ],
        )
        .unwrap();

        let dag = dagger(&mat);
        assert_eq!(dag[[0, 1]], Complex64::new(0.0, 1.0));
        assert_eq!(dag[[1, 0]], Complex64::new(0.0, -1.0));
    }

    #[test]
    fn test_trace_identity() {
        let identity = Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
            ],
        )
        .unwrap();

        let tr = trace(&identity).unwrap();
        assert_relative_eq!(tr.re, 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_ket_to_rho() {
        let ket = Array1::from_vec(vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
        ]);

        let rho = ket_to_rho(&ket);
        assert_eq!(rho[[0, 0]], Complex64::new(1.0, 0.0));
        assert_eq!(rho[[1, 1]], Complex64::new(0.0, 0.0));
    }
}
