//! Statistical testing and parameter fitting

use crate::error::{EngineError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Measurement data from experiments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementData {
    pub observables: HashMap<String, (Vec<f64>, Vec<f64>)>, // name -> (values, uncertainties)
    pub num_shots: usize,
    pub metadata: serde_json::Value,
}

/// Single measurement event (for time-series data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementEvent {
    pub time: f64,
    pub measurement_id: String,
    pub outcome: usize,
    pub count: usize,
}

/// Parameter fitting result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitResult {
    pub best_params: Vec<f64>,
    pub uncertainties: Vec<f64>,
    pub log_likelihood: f64,
    pub fisher_info: Vec<Vec<f64>>,
    pub converged: bool,
    pub iterations: usize,
}

/// Theory testing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub method: String,
    pub statistic: f64,
    pub p_value: Option<f64>,
    pub degrees_of_freedom: Option<usize>,
    pub conclusion: String,
}

/// Statistical testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatsMethod {
    LogLikelihood,
    ChiSquare,
    KLDivergence,
}


/// Load measurement data from CSV
pub fn load_measurements(csv_path: &str) -> Result<MeasurementData> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    let file = File::open(csv_path)?;
    let reader = BufReader::new(file);
    
    let mut observables: HashMap<String, (Vec<f64>, Vec<f64>)> = HashMap::new();
    let mut num_shots = 0;
    
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            return Err(EngineError::Parse(
                format!("Line {}: Expected at least 3 columns (observable,value,uncertainty)", idx + 1)
            ));
        }
        
        let observable = parts[0].trim().to_string();
        let value: f64 = parts[1].trim().parse()
            .map_err(|e| EngineError::Parse(format!("Line {}: Invalid value: {}", idx + 1, e)))?;
        let uncertainty: f64 = parts[2].trim().parse()
            .map_err(|e| EngineError::Parse(format!("Line {}: Invalid uncertainty: {}", idx + 1, e)))?;
        
        let entry = observables.entry(observable).or_insert_with(|| (Vec::new(), Vec::new()));
        entry.0.push(value);
        entry.1.push(uncertainty);
        num_shots += 1;
    }
    
    Ok(MeasurementData {
        observables,
        num_shots,
        metadata: serde_json::json!({}),
    })
}

/// Compute Gaussian log-likelihood for continuous measurements
/// L(θ) = -1/2 Σ_i [(y_i - f(x_i;θ))² / σ_i² + log(2πσ_i²)]
pub fn gaussian_log_likelihood(
    observed_values: &[f64],
    uncertainties: &[f64],
    predicted_values: &[f64],
) -> Result<f64> {
    if observed_values.len() != uncertainties.len() || observed_values.len() != predicted_values.len() {
        return Err(EngineError::Validation(
            "Observed, uncertainties, and predicted must have same length".to_string()
        ));
    }
    
    let mut log_l = 0.0;
    let two_pi = 2.0 * std::f64::consts::PI;
    
    for i in 0..observed_values.len() {
        let residual = observed_values[i] - predicted_values[i];
        let sigma_sq = uncertainties[i] * uncertainties[i];
        
        if sigma_sq <= 0.0 {
            return Err(EngineError::Validation(
                format!("Uncertainty must be positive, got {}", uncertainties[i])
            ));
        }
        
        log_l -= 0.5 * (residual * residual / sigma_sq + (two_pi * sigma_sq).ln());
    }
    
    Ok(log_l)
}

/// Compute log-likelihood for discrete (binomial) measurements
/// L(θ) = Σ_{i,m} n_{i,m} log P(m|θ,t_i)
pub fn log_likelihood(
    observed: &[MeasurementEvent],
    predicted_probs: &HashMap<(f64, usize), f64>,
) -> Result<f64> {
    let mut log_l = 0.0;

    for event in observed {
        let key = (event.time, event.outcome);
        if let Some(&prob) = predicted_probs.get(&key) {
            if prob > 0.0 {
                log_l += (event.count as f64) * prob.ln();
            } else {
                return Err(EngineError::Validation(
                    "Predicted probability is zero for observed event".to_string()
                ));
            }
        } else {
            return Err(EngineError::Validation(format!(
                "No prediction for time={}, outcome={}",
                event.time, event.outcome
            )));
        }
    }

    Ok(log_l)
}

/// Compute chi-square statistic: χ² = Σ (observed - expected)² / expected
pub fn chi_square(
    observed: &[MeasurementEvent],
    predicted_probs: &HashMap<(f64, usize), f64>,
) -> Result<f64> {
    let mut chi2 = 0.0;
    let mut grouped: HashMap<(f64, String), Vec<(usize, usize)>> = HashMap::new();

    // Group by time and measurement_id
    for event in observed {
        let key = (event.time, event.measurement_id.clone());
        grouped
            .entry(key)
            .or_insert_with(Vec::new)
            .push((event.outcome, event.count));
    }

    for ((time, _meas_id), outcomes) in grouped {
        let total: usize = outcomes.iter().map(|(_, count)| count).sum();

        for (outcome, observed_count) in outcomes {
            let key = (time, outcome);
            if let Some(&prob) = predicted_probs.get(&key) {
                let expected = (total as f64) * prob;
                if expected > 0.0 {
                    let diff = (observed_count as f64) - expected;
                    chi2 += (diff * diff) / expected;
                }
            }
        }
    }

    Ok(chi2)
}

/// Compute KL divergence: D_KL(P||Q) = Σ P(x) log(P(x)/Q(x))
pub fn kl_divergence(
    observed: &[MeasurementEvent],
    predicted_probs: &HashMap<(f64, usize), f64>,
) -> Result<f64> {
    let mut kl = 0.0;
    let mut grouped: HashMap<(f64, String), Vec<(usize, usize)>> = HashMap::new();

    for event in observed {
        let key = (event.time, event.measurement_id.clone());
        grouped
            .entry(key)
            .or_insert_with(Vec::new)
            .push((event.outcome, event.count));
    }

    for ((time, _meas_id), outcomes) in grouped {
        let total: usize = outcomes.iter().map(|(_, count)| count).sum();

        for (outcome, observed_count) in outcomes {
            let p = (observed_count as f64) / (total as f64);
            if p > 0.0 {
                let key = (time, outcome);
                if let Some(&q) = predicted_probs.get(&key) {
                    if q > 0.0 {
                        kl += p * (p / q).ln();
                    } else {
                        return Err(EngineError::Validation(
                            "Predicted probability is zero for observed event".to_string()
                        ));
                    }
                }
            }
        }
    }

    Ok(kl)
}

/// Numerical gradient computation using finite differences
pub fn compute_gradient<F>(
    f: F,
    params: &[f64],
    epsilon: f64,
) -> Result<Vec<f64>>
where
    F: Fn(&[f64]) -> Result<f64>,
{
    let mut gradient = vec![0.0; params.len()];
    
    for i in 0..params.len() {
        let mut params_plus = params.to_vec();
        let mut params_minus = params.to_vec();
        
        params_plus[i] += epsilon;
        params_minus[i] -= epsilon;
        
        let f_plus = f(&params_plus)?;
        let f_minus = f(&params_minus)?;
        
        gradient[i] = (f_plus - f_minus) / (2.0 * epsilon);
    }
    
    Ok(gradient)
}

/// Numerical Hessian (Fisher information) using finite differences
pub fn compute_hessian<F>(
    f: F,
    params: &[f64],
    epsilon: f64,
) -> Result<Vec<Vec<f64>>>
where
    F: Fn(&[f64]) -> Result<f64>,
{
    let n = params.len();
    let mut hessian = vec![vec![0.0; n]; n];
    
    let f_center = f(params)?;
    
    for i in 0..n {
        for j in i..n {
            let mut params_pp = params.to_vec();
            let mut params_pm = params.to_vec();
            let mut params_mp = params.to_vec();
            let mut params_mm = params.to_vec();
            
            params_pp[i] += epsilon;
            params_pp[j] += epsilon;
            
            params_pm[i] += epsilon;
            params_pm[j] -= epsilon;
            
            params_mp[i] -= epsilon;
            params_mp[j] += epsilon;
            
            params_mm[i] -= epsilon;
            params_mm[j] -= epsilon;
            
            if i == j {
                // Diagonal element: second derivative
                let f_plus = f(&params_pp)?;
                let f_minus = f(&params_mm)?;
                hessian[i][i] = (f_plus - 2.0 * f_center + f_minus) / (epsilon * epsilon);
            } else {
                // Off-diagonal: mixed partial derivative
                let f_pp = f(&params_pp)?;
                let f_pm = f(&params_pm)?;
                let f_mp = f(&params_mp)?;
                let f_mm = f(&params_mm)?;
                
                let value = (f_pp - f_pm - f_mp + f_mm) / (4.0 * epsilon * epsilon);
                hessian[i][j] = value;
                hessian[j][i] = value; // Symmetric
            }
        }
    }
    
    Ok(hessian)
}

/// Simple gradient descent optimizer
pub fn gradient_descent<F>(
    f: F,
    initial_params: &[f64],
    max_iterations: usize,
    learning_rate: f64,
    tolerance: f64,
) -> Result<FitResult>
where
    F: Fn(&[f64]) -> Result<f64>,
{
    let mut params = initial_params.to_vec();
    let mut log_likelihood = f(&params)?;
    
    for iter in 0..max_iterations {
        // Compute gradient (negative because we maximize log-likelihood)
        let gradient = compute_gradient(&f, &params, 1e-6)?;
        
        // Update parameters (gradient ascent for maximization)
        let mut max_grad = 0.0;
        for i in 0..params.len() {
            params[i] += learning_rate * gradient[i];
            max_grad = max_grad.max(gradient[i].abs());
        }
        
        let new_log_likelihood = f(&params)?;
        
        // Check convergence
        if (new_log_likelihood - log_likelihood).abs() < tolerance && max_grad < tolerance {
            let hessian = compute_hessian(&f, &params, 1e-6)?;
            let uncertainties = hessian.iter()
                .map(|row| {
                    let diag_elem = row.iter().sum::<f64>();
                    if diag_elem > 0.0 { 1.0 / diag_elem.sqrt() } else { 0.0 }
                })
                .collect();
            
            return Ok(FitResult {
                best_params: params,
                uncertainties,
                log_likelihood: new_log_likelihood,
                fisher_info: hessian,
                converged: true,
                iterations: iter + 1,
            });
        }
        
        log_likelihood = new_log_likelihood;
    }
    
    // Did not converge
    Ok(FitResult {
        best_params: params,
        uncertainties: vec![f64::INFINITY; params.len()],
        log_likelihood,
        fisher_info: vec![vec![0.0; params.len()]; params.len()],
        converged: false,
        iterations: max_iterations,
    })
}

/// Fit parameters using MLE (Maximum Likelihood Estimation)
pub fn fit_parameters_mle<F>(
    likelihood_fn: F,
    initial_guess: &[f64],
    max_iterations: usize,
) -> Result<FitResult>
where
    F: Fn(&[f64]) -> Result<f64>,
{
    // Use gradient descent (could be replaced with L-BFGS-B later)
    gradient_descent(
        likelihood_fn,
        initial_guess,
        max_iterations,
        0.01, // learning rate
        1e-6, // tolerance
    )
}

/// Compute confidence intervals from Fisher information (Hessian)
pub fn compute_confidence_intervals(
    fisher_info: &[Vec<f64>],
    confidence_level: f64,
) -> Result<Vec<(f64, f64)>> {
    // For 95% CI, z = 1.96
    let z_score = match confidence_level {
        0.68 => 1.0,
        0.95 => 1.96,
        0.99 => 2.576,
        _ => return Err(EngineError::Validation(
            format!("Unsupported confidence level: {}", confidence_level)
        )),
    };
    
    let n = fisher_info.len();
    let mut intervals = Vec::new();
    
    for i in 0..n {
        // Fisher information = -Hessian of log-likelihood
        // Variance = (Fisher info)^{-1}_{ii}
        // For now, use diagonal approximation
        let variance = if fisher_info[i][i].abs() > 1e-10 {
            1.0 / fisher_info[i][i].abs()
        } else {
            f64::INFINITY
        };
        
        let std_error = variance.sqrt();
        let margin = z_score * std_error;
        
        intervals.push((-margin, margin)); // Relative to best fit
    }
    
    Ok(intervals)
}

/// Bootstrap resampling for non-parametric confidence intervals
pub fn bootstrap_confidence_intervals<F>(
    data_indices: &[usize],
    fit_fn: F,
    n_bootstrap: usize,
    confidence_level: f64,
) -> Result<Vec<(f64, f64)>>
where
    F: Fn(&[usize]) -> Result<Vec<f64>>,
{
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    
    let mut rng = thread_rng();
    let n_data = data_indices.len();
    
    // Storage for bootstrap parameter estimates
    let first_fit = fit_fn(data_indices)?;
    let n_params = first_fit.len();
    let mut bootstrap_params = vec![Vec::new(); n_params];
    
    for _ in 0..n_bootstrap {
        // Resample with replacement
        let mut resampled: Vec<usize> = (0..n_data)
            .map(|_| *data_indices.choose(&mut rng).unwrap())
            .collect();
        
        // Fit parameters on resampled data
        let params = fit_fn(&resampled)?;
        
        for (i, &param) in params.iter().enumerate() {
            bootstrap_params[i].push(param);
        }
    }
    
    // Compute percentiles for confidence intervals
    let alpha = 1.0 - confidence_level;
    let lower_percentile = alpha / 2.0;
    let upper_percentile = 1.0 - alpha / 2.0;
    
    let mut intervals = Vec::new();
    
    for param_samples in &mut bootstrap_params {
        param_samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let lower_idx = (n_bootstrap as f64 * lower_percentile) as usize;
        let upper_idx = (n_bootstrap as f64 * upper_percentile) as usize;
        
        let lower = param_samples[lower_idx];
        let upper = param_samples[upper_idx];
        
        intervals.push((lower, upper));
    }
    
    Ok(intervals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_log_likelihood() {
        let observed = vec![1.0, 2.0, 3.0];
        let uncertainties = vec![0.1, 0.1, 0.1];
        let predicted = vec![1.05, 1.95, 3.02];
        
        let log_l = gaussian_log_likelihood(&observed, &uncertainties, &predicted).unwrap();
        assert!(log_l < 0.0); // Log-likelihood should be negative
    }

    #[test]
    fn test_log_likelihood_discrete() {
        let events = vec![MeasurementEvent {
            time: 0.0,
            measurement_id: "z".to_string(),
            outcome: 0,
            count: 100,
        }];

        let mut predicted = HashMap::new();
        predicted.insert((0.0, 0), 0.9);

        let log_l = log_likelihood(&events, &predicted).unwrap();
        assert!(log_l < 0.0);
    }

    #[test]
    fn test_chi_square_simple() {
        let events = vec![
            MeasurementEvent {
                time: 0.0,
                measurement_id: "z".to_string(),
                outcome: 0,
                count: 90,
            },
            MeasurementEvent {
                time: 0.0,
                measurement_id: "z".to_string(),
                outcome: 1,
                count: 10,
            },
        ];

        let mut predicted = HashMap::new();
        predicted.insert((0.0, 0), 0.9);
        predicted.insert((0.0, 1), 0.1);

        let chi2 = chi_square(&events, &predicted).unwrap();
        assert!(chi2 >= 0.0);
        assert!(chi2 < 1.0); // Should be small for good fit
    }
    
    #[test]
    fn test_gradient_descent() {
        // Minimize f(x) = (x - 2)^2 + (y - 3)^2
        let f = |params: &[f64]| -> Result<f64> {
            let x = params[0];
            let y = params[1];
            Ok(-((x - 2.0).powi(2) + (y - 3.0).powi(2))) // Negative for maximization
        };
        
        let result = gradient_descent(f, &[0.0, 0.0], 1000, 0.1, 1e-6).unwrap();
        
        assert!((result.best_params[0] - 2.0).abs() < 0.1);
        assert!((result.best_params[1] - 3.0).abs() < 0.1);
        assert!(result.converged);
    }
    
    #[test]
    fn test_compute_gradient() {
        let f = |params: &[f64]| -> Result<f64> {
            Ok(params[0].powi(2) + 2.0 * params[1])
        };
        
        let grad = compute_gradient(f, &[1.0, 2.0], 1e-6).unwrap();
        
        // df/dx = 2x, df/dy = 2
        assert!((grad[0] - 2.0).abs() < 1e-4);
        assert!((grad[1] - 2.0).abs() < 1e-4);
    }
}
