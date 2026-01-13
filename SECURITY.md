# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of Quantum Theory Engine seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### Please Do NOT:

- Open a public GitHub issue
- Discuss the vulnerability in public forums or social media
- Exploit the vulnerability for any reason

### Please DO:

1. **Email us**: Send details to security@example.com (replace with actual contact)
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)
3. **Wait**: Allow us time to investigate and fix before public disclosure

## Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 1-7 days
  - High: 7-30 days
  - Medium: 30-90 days
  - Low: Next release cycle

## Security Considerations

### Numerical Stability

- **Risk**: Floating-point errors could accumulate in long simulations
- **Mitigation**: Validation tolerance checks, normalized states, adaptive integrators

### Input Validation

- **Risk**: Malicious DSL files could cause crashes or hang
- **Mitigation**: Parser limits, timeout mechanisms, resource quotas

### Dependency Security

- **Risk**: Vulnerabilities in dependencies (OpenBLAS, HDF5, etc.)
- **Mitigation**: Regular `cargo audit`, dependency updates, CI security scanning

### Data Privacy

- **Risk**: Sensitive experimental data in CSV/HDF5 files
- **Mitigation**: No data is sent externally, local execution only, clear data ownership

## Safe Usage Guidelines

1. **Validate inputs**: Parse unknown DSL files in sandboxed environments
2. **Resource limits**: Set timeouts for long-running simulations
3. **Update regularly**: Keep engine and dependencies up to date
4. **Review code**: Audit any modifications before deployment
5. **Access control**: Restrict who can run simulations in production

## Known Limitations

- **Large Hilbert spaces**: Memory exhaustion possible (use resource limits)
- **Stiff ODEs**: May require specialized integrators (future work)
- **Floating-point**: Inherent precision limits in quantum state fidelity

## Security Features

- ✅ Memory-safe Rust implementation
- ✅ Input validation at parse and type-check stages
- ✅ Quantum constraint validation prevents invalid states
- ✅ No network communication (fully local)
- ✅ Reproducibility manifests for audit trails

## Future Security Enhancements

- [ ] Formal verification of critical kernels
- [ ] Fuzzing for parser robustness
- [ ] Sandboxed execution environment
- [ ] Cryptographic signing of simulation results

---

**Last Updated**: January 13, 2026
