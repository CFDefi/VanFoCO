# Quantum Theory Engine

An industrial-grade quantum simulation and theory-testing framework that allows physicists to write quantum theory equations in a domain-specific language, compile them into executable models, and rigorously test predictions against experimental data.

## Features

- **Physics DSL**: Write quantum theories using an intuitive, equation-based language
- **Rigorous Compilation**: Parse → AST → Type Check → Quantum Validation → IR → Execution
- **Multiple Evolution Methods**: Schrödinger equation (pure states) and Lindblad master equation (open systems)
- **Statistical Testing**: Log-likelihood, χ², KL divergence, MLE parameter fitting with confidence intervals
- **Reproducibility**: Complete manifest tracking for all simulations
- **Performance**: CPU-optimized with GPU backend planned

## Quick Start

### Installation

```bash
# Build the Rust core
cargo build --release

# Install Python bindings
cd python_bindings
pip install -e .
```

### Example: Rabi Oscillations

Create `rabi.phys`:
```
experiment rabi {
  const omega = 1.0;
  const Omega = 0.2;
  
  Hamiltonian H = (omega/2)*sigma_z + Omega*sigma_x;
  
  init: ket([1, 0]);
  evolution: evolve(init, H, timegrid=(0.0, 0.01, 501));
  
  measure z_basis: Projective([
    [[1,0],[0,0]],
    [[0,0],[0,1]]
  ]);
}
```

Run simulation:
```bash
qte run rabi.phys --output results.h5
```

### Python API

```python
from quantum_theory_engine import load_model, run_simulation, fit_parameters

# Load and run
model = load_model("rabi.phys")
results = run_simulation(model, params={"omega": 1.0, "Omega": 0.2})

# Fit to data
fit_result = fit_parameters(model, "measurements.csv", 
                            param_guesses={"omega": 0.9, "Omega": 0.25})
print(f"Fitted parameters: {fit_result.params}")
print(f"Confidence intervals: {fit_result.confidence_intervals}")
```

## Architecture

```
DSL Source → Parser → AST → Type Checker → Quantum Validator → Optimizer → IR → Executor
                                                                                    ↓
                                                                            Simulation Results
                                                                                    ↓
                                                                        Statistical Testing & Fitting
```

### Core Modules

- **Frontend**: Lexer, parser, AST construction
- **Semantic Analysis**: Type checking, quantum constraint validation
- **Optimization**: CSE, constant folding, sparsity detection
- **IR & Lowering**: Intermediate representation and kernel selection
- **Executor**: CPU/GPU backends, ODE integration, measurement evaluation
- **Analytics**: Statistical testing, parameter fitting, confidence intervals

## MVP Scope

- 1-2 qubits (Hilbert dimensions 2 and 4)
- Pure states |ψ⟩ and density matrices ρ
- Time-dependent Hamiltonians H(t)
- Lindblad master equation with single operator
- Projective measurements and POVMs
- MLE fitting with bootstrap confidence intervals

## Build Phases

1. **Phase 1**: Parser & AST (2 weeks)
2. **Phase 2**: Type Checker & Validator (2-3 weeks)
3. **Phase 3**: IR & Executor (3-4 weeks)
4. **Phase 4**: Measurements & I/O (2 weeks)
5. **Phase 5**: Statistics & Fitting (2-3 weeks)
6. **Phase 6**: Optimization (2 weeks)
7. **Phase 7**: Testing & Release (2 weeks)

**Total**: ~12-16 weeks (single developer)

## Tech Stack

- **Core**: Rust (memory safety, performance, correctness)
- **Numerics**: OpenBLAS/LAPACK, custom ODE integrators
- **Sparse**: CSR/COO formats with `sprs`
- **Python**: pyo3 bindings
- **Data**: HDF5 for results, JSON for manifests
- **Future**: CUDA/ROCm GPU backends

## Project Structure

```
quantum-theory-engine/
├── Cargo.toml              # Workspace config
├── crates/
│   └── core_engine/        # Core Rust library
│       ├── src/
│       │   ├── lib.rs
│       │   ├── parser.rs   # Lexer & parser
│       │   ├── ast.rs      # AST types
│       │   ├── typechecker.rs
│       │   ├── validator.rs
│       │   ├── ir.rs       # IR definition
│       │   ├── lowering.rs
│       │   ├── executor.rs
│       │   ├── kernels_cpu.rs
│       │   ├── ode.rs      # ODE integrators
│       │   └── stats.rs    # Statistical testing
│       └── grammar.pest    # PEG grammar
├── cli/
│   └── src/
│       └── main.rs         # CLI tool
├── python_bindings/
│   ├── src/
│   │   └── lib.rs          # pyo3 bindings
│   └── pyproject.toml
├── dsl_examples/
│   ├── rabi.phys
│   └── amp_damp.phys
├── docs/
│   ├── spec/
│   │   └── grammar.ebnf    # Formal grammar
│   └── manifest_schema.json
└── tests/
    └── integration/
```

## License

MIT OR Apache-2.0

## Contributing

This is an MVP implementation. Contributions welcome after initial release.
