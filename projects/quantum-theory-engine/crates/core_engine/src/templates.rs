//! Template library for common quantum systems
//!
//! Provides pre-built, validated DSL programs for standard experiments.

use crate::error::{EngineError, Result};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Template registry
pub struct TemplateRegistry {
    templates: HashMap<String, Template>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub parameters: Vec<TemplateParameter>,
    pub code: String,
    pub tags: Vec<String>,
    pub citations: Vec<Citation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateCategory {
    SingleQubit,
    TwoQubit,
    Cavity,
    OpenSystems,
    ManyBody,
    MetrologyOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    pub name: String,
    pub description: String,
    pub default_value: f64,
    pub constraints: ParameterConstraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterConstraints {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub must_be_positive: bool,
    pub must_be_integer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub authors: String,
    pub title: String,
    pub journal: String,
    pub year: u32,
    pub doi: Option<String>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            templates: HashMap::new(),
        };
        
        registry.load_builtin_templates();
        registry
    }

    fn load_builtin_templates(&mut self) {
        self.register(rabi_oscillations());
        self.register(ramsey_interferometry());
        self.register(bell_state_tomography());
        self.register(jaynes_cummings());
        self.register(quantum_zeno());
        self.register(grover_search());
        self.register(vqe_h2());
    }

    pub fn register(&mut self, template: Template) {
        self.templates.insert(template.id.clone(), template);
    }

    pub fn get(&self, id: &str) -> Option<&Template> {
        self.templates.get(id)
    }

    pub fn list_by_category(&self, category: TemplateCategory) -> Vec<&Template> {
        self.templates.values()
            .filter(|t| t.category == category)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Template> {
        let query = query.to_lowercase();
        self.templates.values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query) ||
                t.description.to_lowercase().contains(&query) ||
                t.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect()
    }

    pub fn instantiate(&self, id: &str, params: &HashMap<String, f64>) -> Result<String> {
        let template = self.get(id)
            .ok_or_else(|| EngineError::NotFound(format!("Template not found: {}", id)))?;

        // Validate parameters
        for param in &template.parameters {
            if let Some(&value) = params.get(&param.name) {
                self.validate_parameter(param, value)?;
            }
        }

        // Substitute parameters in template code
        let mut code = template.code.clone();
        for (name, value) in params {
            let placeholder = format!("{{{}}}", name);
            code = code.replace(&placeholder, &value.to_string());
        }

        Ok(code)
    }

    fn validate_parameter(&self, param: &TemplateParameter, value: f64) -> Result<()> {
        if let Some(min) = param.constraints.min {
            if value < min {
                return Err(EngineError::Validation(
                    format!("Parameter {} = {} is less than minimum {}", param.name, value, min)
                ));
            }
        }

        if let Some(max) = param.constraints.max {
            if value > max {
                return Err(EngineError::Validation(
                    format!("Parameter {} = {} exceeds maximum {}", param.name, value, max)
                ));
            }
        }

        if param.constraints.must_be_positive && value <= 0.0 {
            return Err(EngineError::Validation(
                format!("Parameter {} = {} must be positive", param.name, value)
            ));
        }

        if param.constraints.must_be_integer && value.fract() != 0.0 {
            return Err(EngineError::Validation(
                format!("Parameter {} = {} must be an integer", param.name, value)
            ));
        }

        Ok(())
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Built-in templates

fn rabi_oscillations() -> Template {
    Template {
        id: "rabi".to_string(),
        name: "Rabi Oscillations".to_string(),
        description: "Coherent oscillations of a driven two-level system".to_string(),
        category: TemplateCategory::SingleQubit,
        parameters: vec![
            TemplateParameter {
                name: "omega".to_string(),
                description: "Rabi frequency (MHz)".to_string(),
                default_value: 1.0,
                constraints: ParameterConstraints {
                    min: Some(0.0),
                    max: None,
                    must_be_positive: true,
                    must_be_integer: false,
                },
            },
            TemplateParameter {
                name: "T".to_string(),
                description: "Evolution time (μs)".to_string(),
                default_value: 10.0,
                constraints: ParameterConstraints {
                    min: Some(0.0),
                    max: None,
                    must_be_positive: true,
                    must_be_integer: false,
                },
            },
        ],
        code: r#"
param omega: Real = {omega};  # Rabi frequency
param T: Real = {T};          # Total time

state psi: Qubit = |0>;
operator H: Operator<2> = omega * sigma_x / 2;

evolve psi under H for T;
measure psi in computational;
"#.to_string(),
        tags: vec!["rabi".to_string(), "qubit".to_string(), "driving".to_string()],
        citations: vec![
            Citation {
                authors: "Rabi, I. I.".to_string(),
                title: "Space Quantization in a Gyrating Magnetic Field".to_string(),
                journal: "Physical Review".to_string(),
                year: 1937,
                doi: Some("10.1103/PhysRev.51.652".to_string()),
            },
        ],
    }
}

fn ramsey_interferometry() -> Template {
    Template {
        id: "ramsey".to_string(),
        name: "Ramsey Interferometry".to_string(),
        description: "Two π/2 pulses separated by free evolution for precision frequency measurements".to_string(),
        category: TemplateCategory::SingleQubit,
        parameters: vec![
            TemplateParameter {
                name: "delta".to_string(),
                description: "Detuning from resonance (kHz)".to_string(),
                default_value: 0.1,
                constraints: ParameterConstraints {
                    min: None,
                    max: None,
                    must_be_positive: false,
                    must_be_integer: false,
                },
            },
            TemplateParameter {
                name: "tau".to_string(),
                description: "Free evolution time (μs)".to_string(),
                default_value: 5.0,
                constraints: ParameterConstraints {
                    min: Some(0.0),
                    max: None,
                    must_be_positive: true,
                    must_be_integer: false,
                },
            },
        ],
        code: r#"
param delta: Real = {delta};  # Detuning
param tau: Real = {tau};      # Free evolution

state psi: Qubit = |0>;
operator pi_2: Unitary<2> = expm(-i * pi/4 * sigma_y);
operator H_det: Operator<2> = delta * sigma_z / 2;

apply pi_2 to psi;
evolve psi under H_det for tau;
apply pi_2 to psi;

measure psi in computational;
"#.to_string(),
        tags: vec!["ramsey".to_string(), "interferometry".to_string(), "metrology".to_string()],
        citations: vec![],
    }
}

fn bell_state_tomography() -> Template {
    Template {
        id: "bell_tomography".to_string(),
        name: "Bell State Tomography".to_string(),
        description: "Prepare and measure Bell states in multiple bases".to_string(),
        category: TemplateCategory::TwoQubit,
        parameters: vec![
            TemplateParameter {
                name: "shots".to_string(),
                description: "Number of measurement shots".to_string(),
                default_value: 1000.0,
                constraints: ParameterConstraints {
                    min: Some(1.0),
                    max: None,
                    must_be_positive: true,
                    must_be_integer: true,
                },
            },
        ],
        code: r#"
param shots: Int = {shots};

state psi: Qubit^2 = |00>;

# Prepare Bell state |Φ+⟩ = (|00⟩ + |11⟩)/√2
apply hadamard to psi[0];
apply cnot to psi;

# Tomography in computational basis
measure psi in computational with shots;

# Tomography in X basis (both qubits)
apply hadamard to psi[0];
apply hadamard to psi[1];
measure psi in computational with shots;

# Tomography in Y basis
apply s_dagger to psi[0];
apply hadamard to psi[0];
apply s_dagger to psi[1];
apply hadamard to psi[1];
measure psi in computational with shots;
"#.to_string(),
        tags: vec!["bell".to_string(), "entanglement".to_string(), "tomography".to_string()],
        citations: vec![],
    }
}

fn jaynes_cummings() -> Template {
    Template {
        id: "jaynes_cummings".to_string(),
        name: "Jaynes-Cummings Model".to_string(),
        description: "Atom-cavity coupling with vacuum Rabi oscillations".to_string(),
        category: TemplateCategory::Cavity,
        parameters: vec![
            TemplateParameter {
                name: "g".to_string(),
                description: "Coupling strength (MHz)".to_string(),
                default_value: 1.0,
                constraints: ParameterConstraints {
                    min: Some(0.0),
                    max: None,
                    must_be_positive: true,
                    must_be_integer: false,
                },
            },
            TemplateParameter {
                name: "n_max".to_string(),
                description: "Maximum photon number".to_string(),
                default_value: 5.0,
                constraints: ParameterConstraints {
                    min: Some(1.0),
                    max: Some(20.0),
                    must_be_positive: true,
                    must_be_integer: true,
                },
            },
        ],
        code: r#"
param g: Real = {g};        # Coupling
param n_max: Int = {n_max}; # Truncation

state atom: Qubit = |e>;
state cavity: Fock<n_max> = |0>;

operator sigma_p: Operator<2> = |e><g|;
operator sigma_m: Operator<2> = |g><e|;
operator a: Operator<n_max> = annihilation;
operator a_dag: Operator<n_max> = creation;

operator H_int: Operator<2*n_max> = 
    g * (tensor(sigma_p, a) + tensor(sigma_m, a_dag));

state psi: (Qubit, Fock<n_max>) = tensor(atom, cavity);
evolve psi under H_int for 10.0;

measure atom in computational;
measure cavity in fock;
"#.to_string(),
        tags: vec!["cavity".to_string(), "atom".to_string(), "coupling".to_string()],
        citations: vec![],
    }
}

fn quantum_zeno() -> Template {
    Template {
        id: "zeno".to_string(),
        name: "Quantum Zeno Effect".to_string(),
        description: "Frequent measurements freeze quantum evolution".to_string(),
        category: TemplateCategory::OpenSystems,
        parameters: vec![
            TemplateParameter {
                name: "n_measure".to_string(),
                description: "Number of intermediate measurements".to_string(),
                default_value: 10.0,
                constraints: ParameterConstraints {
                    min: Some(0.0),
                    max: Some(100.0),
                    must_be_positive: true,
                    must_be_integer: true,
                },
            },
        ],
        code: r#"
param omega: Real = 1.0;
param T: Real = 10.0;
param n_measure: Int = {n_measure};

state psi: Qubit = |0>;
operator H: Operator<2> = omega * sigma_x / 2;

for i in 0..n_measure {
    evolve psi under H for T/n_measure;
    measure psi in computational;
    if outcome == |1> {
        set psi = |0>;  # Reset on |1⟩
    }
}

measure psi in computational;
"#.to_string(),
        tags: vec!["zeno".to_string(), "measurement".to_string(), "dynamics".to_string()],
        citations: vec![],
    }
}

fn grover_search() -> Template {
    Template {
        id: "grover".to_string(),
        name: "Grover Search Algorithm".to_string(),
        description: "Quantum search with quadratic speedup".to_string(),
        category: TemplateCategory::ManyBody,
        parameters: vec![
            TemplateParameter {
                name: "n_qubits".to_string(),
                description: "Number of qubits (search space size 2^n)".to_string(),
                default_value: 3.0,
                constraints: ParameterConstraints {
                    min: Some(1.0),
                    max: Some(10.0),
                    must_be_positive: true,
                    must_be_integer: true,
                },
            },
            TemplateParameter {
                name: "target".to_string(),
                description: "Target state index to find".to_string(),
                default_value: 5.0,
                constraints: ParameterConstraints {
                    min: Some(0.0),
                    max: None,
                    must_be_positive: true,
                    must_be_integer: true,
                },
            },
        ],
        code: r#"
param n_qubits: Int = {n_qubits};
param target: Int = {target};

state psi: Qubit^n_qubits = |0>^n_qubits;

# Initialize superposition
for i in 0..n_qubits {
    apply hadamard to psi[i];
}

# Grover iterations (optimal = π/4 * sqrt(2^n))
param n_iter: Int = floor(pi/4 * sqrt(2^n_qubits));

for iter in 0..n_iter {
    # Oracle: flip phase of target state
    apply phase_oracle(target) to psi;
    
    # Diffusion operator
    for i in 0..n_qubits { apply hadamard to psi[i]; }
    apply inversion_about_zero to psi;
    for i in 0..n_qubits { apply hadamard to psi[i]; }
}

measure psi in computational with 1000;
"#.to_string(),
        tags: vec!["grover".to_string(), "search".to_string(), "algorithm".to_string()],
        citations: vec![],
    }
}

fn vqe_h2() -> Template {
    Template {
        id: "vqe_h2".to_string(),
        name: "VQE for H₂ Molecule".to_string(),
        description: "Variational quantum eigensolver for hydrogen molecule".to_string(),
        category: TemplateCategory::MetrologyOptimization,
        parameters: vec![
            TemplateParameter {
                name: "bond_length".to_string(),
                description: "H-H bond length (Angstroms)".to_string(),
                default_value: 0.74,
                constraints: ParameterConstraints {
                    min: Some(0.3),
                    max: Some(2.0),
                    must_be_positive: true,
                    must_be_integer: false,
                },
            },
        ],
        code: r#"
param R: Real = {bond_length};  # Bond length

# Hamiltonian in Pauli basis (from PySCF/OpenFermion)
operator H: Operator<4> = 
    -1.0523 * identity +
    0.3979 * (Z0 ⊗ Z1) +
    -0.3979 * (Z2 ⊗ Z3) +
    -0.0112 * (Z0 ⊗ Z2) +
    0.1809 * (X0 ⊗ X1 ⊗ Y2 ⊗ Y3);

# Ansatz: Hardware-efficient trial state
param theta: Real[6];  # Variational parameters

state psi: Qubit^4 = |0000>;

# Layer 1
for i in 0..4 { apply ry(theta[0]) to psi[i]; }
apply cnot to psi[0:1];
apply cnot to psi[2:3];

# Layer 2
for i in 0..4 { apply ry(theta[1]) to psi[i]; }
apply cnot to psi[1:2];

measure H on psi;  # Expectation value

# Fit to find optimal theta minimizing ⟨H⟩
fit theta to minimize expectation(H);
"#.to_string(),
        tags: vec!["vqe".to_string(), "optimization".to_string(), "chemistry".to_string()],
        citations: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_registry() {
        let registry = TemplateRegistry::new();
        assert!(registry.get("rabi").is_some());
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_template_instantiation() {
        let registry = TemplateRegistry::new();
        let mut params = HashMap::new();
        params.insert("omega".to_string(), 2.0);
        params.insert("T".to_string(), 5.0);

        let code = registry.instantiate("rabi", &params).unwrap();
        assert!(code.contains("omega: Real = 2"));
        assert!(code.contains("T: Real = 5"));
    }

    #[test]
    fn test_parameter_validation() {
        let registry = TemplateRegistry::new();
        let template = registry.get("rabi").unwrap();
        let param = &template.parameters[0];

        assert!(registry.validate_parameter(param, 1.0).is_ok());
        assert!(registry.validate_parameter(param, -1.0).is_err());
    }

    #[test]
    fn test_category_filtering() {
        let registry = TemplateRegistry::new();
        let single_qubit = registry.list_by_category(TemplateCategory::SingleQubit);
        assert!(!single_qubit.is_empty());
    }
}
