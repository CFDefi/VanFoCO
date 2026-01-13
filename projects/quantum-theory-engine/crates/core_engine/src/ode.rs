//! ODE integrators for quantum evolution

use crate::error::{EngineError, Result};
use ndarray::Array2;
use num_complex::Complex64;

/// ODE integration result
pub struct OdeResult {
    pub times: Vec<f64>,
    pub states: Vec<Array2<Complex64>>,
}

/// RK4 (4th order Runge-Kutta) integrator for Lindblad master equation
pub struct Rk4Integrator {
    hamiltonian: Array2<Complex64>,
    lindblad_ops: Vec<(Array2<Complex64>, f64)>, // (L, gamma)
}

impl Rk4Integrator {
    pub fn new(
        hamiltonian: Array2<Complex64>,
        lindblad_ops: Vec<(Array2<Complex64>, f64)>,
    ) -> Self {
        Rk4Integrator {
            hamiltonian,
            lindblad_ops,
        }
    }

    /// Integrate from initial density matrix over time grid
    pub fn integrate(
        &self,
        initial_rho: Array2<Complex64>,
        times: &[f64],
    ) -> Result<OdeResult> {
        if times.is_empty() {
            return Err(EngineError::validation_error("Empty time grid"));
        }

        let mut states = Vec::with_capacity(times.len());
        let mut rho = initial_rho;
        states.push(rho.clone());

        for i in 1..times.len() {
            let dt = times[i] - times[i - 1];
            if dt <= 0.0 {
                return Err(EngineError::validation_error("Time grid must be increasing"));
            }

            rho = self.step_rk4(&rho, dt)?;
            states.push(rho.clone());
        }

        Ok(OdeResult {
            times: times.to_vec(),
            states,
        })
    }

    /// Single RK4 step
    fn step_rk4(&self, rho: &Array2<Complex64>, dt: f64) -> Result<Array2<Complex64>> {
        let k1 = self.lindblad_derivative(rho)?;
        let rho2 = rho + &k1 * Complex64::new(dt / 2.0, 0.0);

        let k2 = self.lindblad_derivative(&rho2)?;
        let rho3 = rho + &k2 * Complex64::new(dt / 2.0, 0.0);

        let k3 = self.lindblad_derivative(&rho3)?;
        let rho4 = rho + &k3 * Complex64::new(dt, 0.0);

        let k4 = self.lindblad_derivative(&rho4)?;

        // rho_new = rho + (dt/6) * (k1 + 2*k2 + 2*k3 + k4)
        let result = rho
            + &((&k1 + &(&k2 * Complex64::new(2.0, 0.0)) + &(&k3 * Complex64::new(2.0, 0.0)) + &k4)
                * Complex64::new(dt / 6.0, 0.0));

        Ok(result)
    }

    /// Lindblad master equation: dρ/dt = -i[H,ρ] + Σ_k γ_k (L_k ρ L_k† - 1/2{L_k†L_k, ρ})
    fn lindblad_derivative(&self, rho: &Array2<Complex64>) -> Result<Array2<Complex64>> {
        let n = rho.nrows();
        let mut drho = Array2::zeros((n, n));

        // Unitary part: -i[H, ρ]
        let h_rho = self.hamiltonian.dot(rho);
        let rho_h = rho.dot(&self.hamiltonian);
        let commutator = h_rho - rho_h;
        drho = drho + commutator * Complex64::new(0.0, -1.0);

        // Dissipative part
        for (l_op, gamma) in &self.lindblad_ops {
            let l_dagger = l_op.t().mapv(|x| x.conj());

            // L ρ L†
            let l_rho = l_op.dot(rho);
            let l_rho_l_dag = l_rho.dot(&l_dagger);

            // L†L
            let l_dag_l = l_dagger.dot(l_op);

            // {L†L, ρ} = L†Lρ + ρL†L
            let anticomm = l_dag_l.dot(rho) + rho.dot(&l_dag_l);

            // γ(LρL† - 1/2{L†L,ρ})
            let dissipator = (l_rho_l_dag - anticomm * Complex64::new(0.5, 0.0))
                * Complex64::new(*gamma, 0.0);

            drho = drho + dissipator;
        }

        Ok(drho)
    }
}

/// Unitary evolution for pure states: |ψ(t)⟩ = U(t)|ψ(0)⟩ where U(t) = exp(-iHt)
pub fn evolve_unitary(
    hamiltonian: &Array2<Complex64>,
    initial_ket: &ndarray::Array1<Complex64>,
    times: &[f64],
) -> Result<Vec<ndarray::Array1<Complex64>>> {
    use crate::kernels_cpu::{apply_unitary_ket, matrix_exp};

    let mut states = Vec::with_capacity(times.len());
    states.push(initial_ket.clone());

    for i in 1..times.len() {
        let dt = times[i] - times[i - 1];

        // U(dt) = exp(-i H dt)
        let h_dt = hamiltonian * Complex64::new(0.0, -dt);
        let u = matrix_exp(&h_dt)?;

        let new_state = apply_unitary_ket(&u, &states[i - 1])?;
        states.push(new_state);
    }

    Ok(states)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_rk4_constant_state() {
        // With H=0 and no Lindblad ops, state should remain constant
        let h = Array2::zeros((2, 2));
        let rho0 = Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
            ],
        )
        .unwrap();

        let integrator = Rk4Integrator::new(h, Vec::new());
        let times = vec![0.0, 0.1, 0.2];
        let result = integrator.integrate(rho0.clone(), &times).unwrap();

        assert_relative_eq!(
            result.states[2][[0, 0]].re,
            1.0,
            epsilon = 1e-6
        );
    }
}
