# Contributing to Quantum Theory Engine

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Development Setup

1. **Install Rust**: https://rustup.rs
2. **Install dependencies**:
   ```bash
   # macOS
   brew install openblas hdf5
   
   # Ubuntu/Debian
   sudo apt-get install libopenblas-dev libhdf5-dev
   ```
3. **Clone and build**:
   ```bash
   git clone <repository-url>
   cd quantum-theory-engine
   cargo build
   cargo test
   ```

## Contribution Workflow

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/your-feature-name`
3. **Make changes** following our coding standards
4. **Write tests** for new functionality
5. **Run validation**: `./scripts/validate.sh`
6. **Commit** with clear messages
7. **Push** and create a Pull Request

## Coding Standards

### Rust Code

- **Format**: Run `cargo fmt` before committing
- **Lint**: Ensure `cargo clippy` passes with no warnings
- **Tests**: All public APIs must have unit tests
- **Documentation**: Public items must have doc comments (`///`)
- **Error handling**: Use `Result<T, EngineError>` consistently

Example:
```rust
/// Compute the matrix exponential using eigendecomposition
///
/// # Arguments
/// * `mat` - Square complex matrix
///
/// # Returns
/// * `Ok(exp_mat)` - Matrix exponential
/// * `Err` - If matrix is not square or eigendecomposition fails
///
/// # Example
/// ```
/// let mat = Array2::eye(2);
/// let exp_mat = matrix_exp(&mat)?;
/// ```
pub fn matrix_exp(mat: &Array2<Complex64>) -> Result<Array2<Complex64>> {
    // Implementation
}
```

### DSL Grammar

- Follow existing EBNF specification
- Add parser tests for new syntax
- Update documentation in `docs/spec/grammar.ebnf`

### Python Bindings

- Use type hints consistently
- Follow PEP 8 style guide
- Add docstrings with examples

## Areas for Contribution

### High Priority

1. **Complete Python bindings** (pyo3)
   - Wrap core APIs
   - Add numpy integration
   - Create example notebooks

2. **Implement MLE fitting**
   - Integrate `argmin` optimizer
   - Add adjoint method for gradients
   - Bootstrap confidence intervals

3. **Optimization passes**
   - Common subexpression elimination
   - Algebraic simplifications
   - Sparse matrix detection

4. **Additional examples**
   - Multi-qubit systems
   - Decoherence models
   - Quantum gates library

### Medium Priority

1. **GPU backend**
   - cuBLAS/cuSOLVER integration
   - Kernel optimization
   - Benchmarking

2. **Advanced integrators**
   - Magnus expansion
   - Krylov subspace methods
   - Adaptive error control

3. **Measurement improvements**
   - Weak measurements
   - Continuous monitoring
   - Partial trace operations

### Documentation

- Tutorial notebooks
- API reference improvements
- Physics background explanations
- Performance optimization guide

## Testing Guidelines

### Unit Tests

Place in the same file as the code:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_matrix_exp_identity() {
        let identity = Array2::eye(2);
        let result = matrix_exp(&identity).unwrap();
        // assertions
    }
}
```

### Integration Tests

Place in `tests/` directory:
```rust
#[test]
fn test_full_rabi_simulation() {
    let source = std::fs::read_to_string("dsl_examples/rabi.phys").unwrap();
    let ast = parse_dsl(&source).unwrap();
    // ... full pipeline test
}
```

### Property-Based Tests

Use `proptest` for randomized testing:
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn hermitian_eigenvalues_real(mat in hermitian_matrix_strategy()) {
        let eigs = eigenvalues(&mat).unwrap();
        prop_assert!(eigs.iter().all(|e| e.im.abs() < 1e-10));
    }
}
```

## Performance Guidelines

- Profile before optimizing: `cargo flamegraph`
- Benchmark changes: `cargo bench`
- Use `--release` for production builds
- Consider sparse matrices for large systems
- Cache expensive computations

## Documentation

- Update `README.md` for user-facing changes
- Update `CHANGELOG.md` with all changes
- Add doc comments for public APIs
- Create examples for new features

## Pull Request Process

1. **Ensure CI passes**: All tests, formatting, and clippy checks
2. **Update documentation**: README, API docs, examples
3. **Add changelog entry**: Under "Unreleased" section
4. **Request review**: Tag maintainers
5. **Address feedback**: Respond to all comments
6. **Squash commits**: Before merge (if requested)

## Code Review Checklist

- [ ] Tests pass locally and in CI
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Documentation updated
- [ ] Changelog entry added
- [ ] Examples work correctly
- [ ] Performance benchmarks (if applicable)
- [ ] Breaking changes clearly noted

## Questions?

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: File bugs or feature requests
- **Security**: Email security@example.com for vulnerabilities

## License

By contributing, you agree that your contributions will be licensed under both MIT and Apache-2.0 licenses.

---

Thank you for contributing to quantum theory engine! 🚀⚛️
