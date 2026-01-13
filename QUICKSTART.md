# Quantum Theory Engine - Quick Start Guide

## 🚀 Installation (5 minutes)

### Step 1: Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 2: Install Dependencies (macOS)
```bash
brew install openblas hdf5
```

### Step 3: Build the Project
```bash
cd "itunea hCK"
cargo build --release
```

## 🧪 Try It Out (2 minutes)

### Parse a DSL File
```bash
./target/release/qte-cli parse dsl_examples/rabi.phys
```

**Expected output:**
```
Parsing file: dsl_examples/rabi.phys
✓ Parse successful
Statements: 7
```

### Run a Simulation
```bash
./target/release/qte-cli run dsl_examples/rabi.phys
```

**Expected output:**
```
Running simulation: dsl_examples/rabi.phys
✓ Parsed
✓ Type checked
✓ Quantum validated
✓ Lowered to IR
✓ Executed

Results:
  Experiment: rabi
  Time points: 501
  State type: PureState
```

## 📖 Understanding the DSL

### Example 1: Simple Rabi Oscillation

```dsl
// Define physical constants
const omega = 1.0;      // Natural frequency
const Omega = 0.2;      // Rabi frequency

// Define Pauli matrices
matrix sigma_z = [[1, 0], [0, -1]];
matrix sigma_x = [[0, 1], [1, 0]];

// Define Hamiltonian
Hamiltonian H = (omega/2) * sigma_z + Omega * sigma_x;

// Define measurement basis
measure z_basis: Projective([
  [[1,0],[0,0]],  // |0⟩⟨0|
  [[0,0],[0,1]]   // |1⟩⟨1|
]);

// Define experiment
experiment rabi {
  init: ket(vec(1, 0));  // Start in |0⟩
  evolution: evolve(init, H, timegrid=(0.0, 0.01, 501));
  measurements: [(0.0, z_basis), (1.0, z_basis), (2.0, z_basis)];
}
```

### Example 2: Open System Evolution

```dsl
const gamma = 0.1;  // Decay rate

// Lowering operator
matrix L = [[0, 1], [0, 0]];

Hamiltonian H = (omega/2) * sigma_z;

experiment decay {
  init: rho([[0, 0], [0, 1]]);  // Excited state density matrix
  
  // Lindblad master equation
  evolution: evolve(init, H, timegrid=(0.0, 0.05, 401), Lindblad(L, gamma));
  
  measurements: [(0.0, z_basis), (5.0, z_basis), (10.0, z_basis)];
}
```

## 🔬 Test the Pipeline

### Run Unit Tests
```bash
cargo test
```

### Run Integration Tests
```bash
cargo test --test parser_tests
cargo test --test pipeline_tests
```

## 📊 Work with Data

### Load Measurement Data (CSV format)
```csv
time,measurement_id,outcome,count
0.0,z_basis,0,485
0.0,z_basis,1,15
1.0,z_basis,0,352
1.0,z_basis,1,148
```

### Test Theory Against Data (Coming Soon)
```bash
./target/release/qte-cli test dsl_examples/rabi.phys \
  --data dsl_examples/rabi_measurements.csv \
  --method chi-square
```

## 🐍 Python API (In Progress)

```python
from quantum_theory_engine import load_model, run_simulation

# Load model
model = load_model("dsl_examples/rabi.phys")

# Run simulation
params = {"omega": 1.0, "Omega": 0.2}
result = run_simulation(model, params)

# Access results
print(f"Simulation completed with {len(result.times)} time points")
```

## 🛠️ Development Commands

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Build documentation
cargo doc --open

# Run benchmarks (when available)
cargo bench

# Build Python bindings (when implemented)
cd python_bindings
maturin develop
```

## 📝 Next Steps

1. **Try the Examples**: Modify [rabi.phys](dsl_examples/rabi.phys) and re-run
2. **Create Your Own**: Write a new DSL file for your quantum system
3. **Add Measurements**: Extend experiments with POVM measurements
4. **Fit Parameters**: Once implemented, fit your model to experimental data

## 🆘 Troubleshooting

### Build Errors

**"cannot find -lopenblas"**
```bash
brew install openblas
export LIBRARY_PATH=/opt/homebrew/opt/openblas/lib:$LIBRARY_PATH
```

**"hdf5.h not found"**
```bash
brew install hdf5
export HDF5_DIR=/opt/homebrew/opt/hdf5
```

### Parse Errors

Check your DSL syntax against [grammar.ebnf](docs/spec/grammar.ebnf)

### Validation Errors

- Hamiltonians must be Hermitian (H = H†)
- Density matrices must be PSD and have trace = 1
- States must be normalized
- Projectors must sum to identity

## 📚 Documentation

- **Full README**: [README.md](README.md)
- **Build Guide**: [BUILD.md](BUILD.md)
- **Project Summary**: [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)
- **Grammar Spec**: [docs/spec/grammar.ebnf](docs/spec/grammar.ebnf)
- **API Docs**: Run `cargo doc --open`

## 💡 Examples to Try

1. **Two-qubit system**: Use `tensor()` to create entangled states
2. **Time-dependent Hamiltonian**: Define `H(t)` as a function
3. **Multiple Lindblad operators**: Add dephasing + decay
4. **POVM measurements**: Define general measurement operators

## 🎯 Goal

Build and test quantum theories rigorously against experimental data with:
- ✅ Correct quantum mechanics (validated constraints)
- ✅ Reproducible simulations (manifests + checksums)
- ✅ Statistical rigor (MLE, confidence intervals)
- ✅ Industrial performance (BLAS, planned GPU)

---

**Ready to simulate quantum systems!** 🔬⚛️
