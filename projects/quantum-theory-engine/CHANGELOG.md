# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete DSL grammar with Pest parser
- AST representation for quantum programs
- Type checker with shape/dimension validation
- Quantum validator (Hermiticity, PSD, trace, CPTP checks)
- IR lowering and optimization framework
- CPU kernels for linear algebra operations
- RK4 integrator for Lindblad master equation
- Unitary evolution for pure states
- Measurement probability computation
- Statistical testing framework (log-likelihood, chi-square, KL divergence)
- CLI tool with parse/run/fit/test commands
- DSL examples: Rabi oscillations, amplitude damping, Bell states, Jaynes-Cummings
- Docker and docker-compose support
- CI/CD with GitHub Actions
- Comprehensive documentation (README, BUILD, QUICKSTART, PROJECT_SUMMARY)
- Python API stubs (full bindings TODO)
- Reproducibility manifest schema
- Integration test suite

### Changed
- N/A (initial release)

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - 2026-01-13

### Added
- Initial project structure
- Core Rust library scaffolding
- Basic parser and AST definitions
- Error handling framework
- Example DSL files

---

## Version Numbering

- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality in a backwards compatible manner
- **PATCH**: Backwards compatible bug fixes

## Categories

- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Vulnerability fixes
