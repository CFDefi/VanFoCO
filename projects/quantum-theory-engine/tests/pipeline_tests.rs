//! Integration tests for full compilation pipeline

use quantum_theory_engine::{
    parse_dsl, BackendConfig, Executor, TypeChecker, QuantumValidator,
};

#[test]
fn test_full_pipeline_simple() {
    let source = r#"
        const omega = 1.0;
        matrix sigma_z = [[1, 0], [0, -1]];
        Hamiltonian H = (omega/2) * sigma_z;
        
        experiment simple {
            init: ket(vec(1, 0));
        }
    "#;

    // Parse
    let ast = parse_dsl(source).expect("Parse failed");

    // Type check
    let mut typechecker = TypeChecker::new();
    let typed_ast = typechecker.check(&ast).expect("Type check failed");

    // Validate
    let mut validator = QuantumValidator::new();
    let _validated = validator.validate(&typed_ast).expect("Validation failed");

    // If we get here, pipeline succeeded
}

#[test]
fn test_pipeline_rejects_non_hermitian() {
    let source = r#"
        matrix non_hermitian = [[1, 2], [0, 1]];
        Hamiltonian H = non_hermitian;
    "#;

    let ast = parse_dsl(source).expect("Parse failed");
    let mut typechecker = TypeChecker::new();
    let typed_ast = typechecker.check(&ast).expect("Type check failed");

    let mut validator = QuantumValidator::new();
    let result = validator.validate(&typed_ast);

    // Should fail validation
    assert!(result.is_err());
}

#[test]
fn test_pipeline_dimension_mismatch() {
    let source = r#"
        matrix A = [[1, 0]];
        matrix B = [[1], [0], [1]];
        matrix C = A * B;
    "#;

    let ast = parse_dsl(source).expect("Parse failed");
    let mut typechecker = TypeChecker::new();
    let result = typechecker.check(&ast);

    // Should fail type check due to dimension mismatch
    assert!(result.is_err());
}
