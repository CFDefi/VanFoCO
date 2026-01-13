# Quantum Theory Engine - User Tutorial

Welcome to the Quantum Theory Engine! This tutorial will guide you through the basics of simulating, analyzing, and validating quantum systems.

## Installation

### Prerequisites
- Rust 1.75 or later
- Python 3.8+ (optional, for Python bindings)
- Docker (optional, for containerized deployment)

### Build from Source
```bash
git clone https://github.com/yourusername/quantum-theory-engine.git
cd quantum-theory-engine
cargo build --release
```

The CLI binary will be at `target/release/qte`.

### Using Docker
```bash
docker-compose up -d qte-server
docker exec -it qte-server qte --help
```

### Install Python Bindings
```bash
cd python_bindings
pip install maturin
maturin develop --release
```

## Quick Start

### 1. Your First Simulation: Rabi Oscillations

Create a file `rabi.phys`:
```
system RabiOscillations {
    param omega = 1.0;
    param t_max = 10.0;
    
    space = qubit("atom");
    
    state psi_0 = |0>;
    
    hamiltonian H = (omega/2) * sigma_x;
    
    evolve schrodinger(psi_0, H, t_max);
}
```

Run the simulation:
```bash
qte simulate rabi.phys --output rabi_results.json
```

### 2. Using Templates

List available templates:
```bash
qte templates
```

Output:
```
[rabi] Rabi Oscillations - Coherent oscillation of a two-level system
[ramsey] Ramsey Interferometry - Precision metrology with free precession
[bell] Bell State Tomography - Measure two-qubit entanglement
...
```

Instantiate a template:
```bash
qte simulate rabi --param omega=2.5 --param T=20.0 --output results.json
```

### 3. Parameter Fitting

Given experimental data in `data.csv`:
```csv
time,probability
0.0,0.0
1.0,0.84
2.0,0.91
3.0,0.14
4.0,0.01
```

Fit the Rabi frequency:
```bash
qte fit rabi --data data.csv --param omega --initial 1.0 --output fit_results.json
```

Output:
```
Fitting: rabi
✓ Loaded 5 measurements
✓ Converged: true
  omega: 1.573 ± 0.082
```

### 4. Parameter Sweeps

Run a grid scan over coupling strengths:
```bash
qte sweep rabi \
    --range omega:0.5:3.0:20 \
    --workers 4 \
    --output sweep_results.json
```

This runs 20 simulations in parallel with 4 workers.

### 5. Symbolic Proofs

Prove quantum identities:
```bash
qte prove "commutator(sigma_x, sigma_y) == 2i * sigma_z" \
    --max-depth 15 \
    --certificate proof.json
```

Output:
```
Proving: commutator(sigma_x, sigma_y) == 2i * sigma_z
✓ Proof found
```

### 6. Server Mode

Start the job queue server:
```bash
qte server --workers 8 --port 8080
```

This starts an HTTP API at `localhost:8080` for submitting jobs programmatically.

### 7. Health Monitoring

Check system status:
```bash
qte health --detailed
```

Output:
```
Status: HEALTHY
  ✓ engine_initialized
  ✓ memory_available
  ✓ workers_ready
  simulation_count: 142 calls, avg 245ms
  proof_time: 37 calls, avg 1.2s
```

## Python API

```python
import quantum_theory_engine as qte
import numpy as np

# Use templates
registry = qte.TemplateRegistry()
template = registry.get("rabi")
code = template.instantiate({"omega": 1.5, "T": 10.0})

# Run simulation
result = qte.execute(code)
print(f"Simulation completed: {result}")

# Fit parameters
times = np.array([0.0, 1.0, 2.0, 3.0, 4.0])
probs = np.array([0.0, 0.84, 0.91, 0.14, 0.01])

def likelihood(params):
    omega = params[0]
    # Model: P(t) = sin^2(omega * t / 2)
    pred = np.sin(omega * times / 2)**2
    return -np.sum((probs - pred)**2)

result = qte.fit_mle(likelihood, [1.0], max_iter=100)
print(f"Fitted omega = {result['best_params'][0]:.3f} ± {result['uncertainties'][0]:.3f}")

# Job queue
queue = qte.JobQueue(workers=4)
job_id = queue.submit_simulate(code)
job = queue.get_job(job_id)
print(f"Job status: {job.status}")
```

## Advanced Topics

### Custom Hamiltonians

Define multi-qubit systems:
```
system TwoQubitGate {
    param J = 0.5;
    param t = 1.0;
    
    space = qubit("q1") * qubit("q2");
    
    state psi_0 = |01>;
    
    hamiltonian H = J * (sigma_x(q1) * sigma_x(q2) + 
                         sigma_y(q1) * sigma_y(q2));
    
    evolve schrodinger(psi_0, H, t);
}
```

### Lindblad Master Equation

Include dissipation:
```
system DampedOscillator {
    param omega = 1.0;
    param gamma = 0.1;
    param t_max = 50.0;
    
    space = qubit("atom");
    state rho_0 = |+><+|;
    
    hamiltonian H = omega * sigma_z / 2;
    lindblad L = [sqrt(gamma) * sigma_minus];
    
    evolve master_equation(rho_0, H, L, t_max);
}
```

### Expectation Values

Compute observables:
```
observable magnetization = <psi | sigma_z | psi>;
observable energy = <psi | H | psi>;

measure magnetization every 0.1 from 0 to t_max;
```

## Troubleshooting

### Build Errors

If you encounter linker errors:
```bash
# On macOS:
xcode-select --install

# On Ubuntu/Debian:
sudo apt-get install build-essential pkg-config libssl-dev

# On Windows:
# Install Visual Studio Build Tools
```

### Performance Issues

For large parameter sweeps, increase workers:
```bash
qte sweep model.phys --range param:0:10:1000 --workers 16
```

Enable multi-threading in environment:
```bash
export RAYON_NUM_THREADS=16
```

### Memory Constraints

For large Hilbert spaces (N > 10 qubits), use sparse representations or tensor network methods (future feature).

## Next Steps

- Explore all [templates](TEMPLATES.md)
- Read the [DSL Reference](DSL_REFERENCE.md)
- See [API Documentation](https://docs.rs/quantum-theory-engine)
- Check [Examples](dsl_examples/) directory
- Join the community on [GitHub Discussions](https://github.com/yourusername/quantum-theory-engine/discussions)

## Getting Help

- **Documentation**: Run `qte --help` for command reference
- **Validation**: Use `qte validate model.phys` to check syntax
- **Health**: Run `qte health --detailed` to diagnose issues
- **Logs**: Set `RUST_LOG=debug` for verbose output

Happy simulating! 🚀
