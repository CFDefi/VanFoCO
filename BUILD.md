# Quantum Theory Engine - Build Instructions

## Prerequisites

### 1. Install Rust Toolchain
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install System Dependencies (macOS)
```bash
# Install OpenBLAS (for linear algebra)
brew install openblas

# Install HDF5 (for data storage)
brew install hdf5
```

### 3. Install Python (for bindings)
```bash
# Ensure Python 3.8+ is installed
python3 --version

# Install maturin (for building Python bindings)
pip install maturin
```

## Build Instructions

### Build Rust Core Library
```bash
cd "itunea hCK"
cargo build --release
```

### Run Tests
```bash
cargo test
```

### Build CLI Tool
```bash
cargo build --release --bin qte-cli
```

### Run CLI
```bash
# Parse a DSL file
./target/release/qte-cli parse dsl_examples/rabi.phys

# Run a simulation
./target/release/qte-cli run dsl_examples/rabi.phys --output results.h5
```

### Build Python Bindings
```bash
cd python_bindings
maturin develop  # For development
# OR
maturin build --release  # For production wheel
pip install .
```

### Use Python API
```python
from quantum_theory_engine import load_model, run_simulation

# Load model
model = load_model("dsl_examples/rabi.phys")

# Run simulation
params = {"omega": 1.0, "Omega": 0.2}
result = run_simulation(model, params)

print(result.to_dict())
```

## Development Workflow

### 1. Check code formatting
```bash
cargo fmt --check
```

### 2. Run linter
```bash
cargo clippy -- -D warnings
```

### 3. Run benchmarks
```bash
cargo bench
```

## Project Structure

```
quantum-theory-engine/
├── Cargo.toml              # Workspace configuration
├── crates/
│   └── core_engine/        # Core Rust library
│       ├── src/
│       │   ├── lib.rs      # Main library entry
│       │   ├── parser.rs   # DSL parser
│       │   ├── ast.rs      # AST definitions
│       │   ├── typechecker.rs
│       │   ├── validator.rs
│       │   ├── ir.rs       # Intermediate representation
│       │   ├── lowering.rs
│       │   ├── optimizer.rs
│       │   ├── kernels_cpu.rs
│       │   ├── ode.rs      # ODE integrators
│       │   ├── executor.rs
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
│   ├── amp_damp.phys
│   └── rabi_measurements.csv
├── docs/
│   ├── spec/
│   │   └── grammar.ebnf
│   └── manifest_schema.json
└── tests/
```

## Troubleshooting

### Error: "cannot find -lopenblas"
Install OpenBLAS:
```bash
brew install openblas
export LIBRARY_PATH=/opt/homebrew/opt/openblas/lib:$LIBRARY_PATH
```

### Error: "hdf5.h not found"
Install HDF5:
```bash
brew install hdf5
export HDF5_DIR=/opt/homebrew/opt/hdf5
```

### Python bindings fail to build
Ensure maturin is installed:
```bash
pip install --upgrade maturin
```

## Next Steps

1. **Extend DSL**: Add more quantum operations (controlled gates, SWAP, etc.)
2. **Optimize Performance**: Implement sparse matrix support and GPU kernels
3. **Add More Integrators**: Implement Magnus expansion, Krylov methods
4. **Implement MLE Fitting**: Connect to `argmin` optimizer
5. **Create Jupyter Notebooks**: Add tutorial notebooks with examples
6. **Add Visualization**: Plot state evolution, Bloch sphere, measurement distributions

## Performance Tips

- Use `--release` flag for production builds (10-100x faster)
- For large Hilbert spaces (>4 dimensions), enable sparse matrix optimizations
- GPU backend (planned) will provide 10-100x speedup for dense operations
- Profile with `cargo flamegraph` to identify bottlenecks

## Documentation

- API docs: `cargo doc --open`
- User guide: See `docs/` directory
- DSL reference: `docs/spec/grammar.ebnf`

## License

MIT OR Apache-2.0
