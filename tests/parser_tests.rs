//! Integration tests for the parser

use quantum_theory_engine::{parse_dsl, Ast};

#[test]
fn test_parse_const_declaration() {
    let source = "const omega = 1.5;";
    let result = parse_dsl(source);
    assert!(result.is_ok());
    let ast = result.unwrap();
    assert_eq!(ast.statements.len(), 1);
}

#[test]
fn test_parse_matrix_declaration() {
    let source = r#"
        matrix sigma_x = [[0, 1], [1, 0]];
    "#;
    let result = parse_dsl(source);
    assert!(result.is_ok());
}

#[test]
fn test_parse_hamiltonian() {
    let source = r#"
        const omega = 1.0;
        matrix sigma_z = [[1, 0], [0, -1]];
        Hamiltonian H = (omega/2) * sigma_z;
    "#;
    let result = parse_dsl(source);
    assert!(result.is_ok());
}

#[test]
fn test_parse_experiment() {
    let source = r#"
        experiment test {
            init: ket(vec(1, 0));
        }
    "#;
    let result = parse_dsl(source);
    assert!(result.is_ok());
}

#[test]
fn test_parse_rabi_example() {
    let source = std::fs::read_to_string("dsl_examples/rabi.phys")
        .expect("Failed to read rabi.phys");
    let result = parse_dsl(&source);
    assert!(result.is_ok(), "Failed to parse rabi.phys: {:?}", result.err());
}

#[test]
fn test_parse_invalid_syntax() {
    let source = "const omega = ;"; // Missing value
    let result = parse_dsl(source);
    assert!(result.is_err());
}
