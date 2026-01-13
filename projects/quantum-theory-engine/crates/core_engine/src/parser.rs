//! Parser for the quantum DSL using Pest PEG parser

use crate::ast::*;
use crate::error::{EngineError, Result};
use num_complex::Complex64;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DslParser;

/// Parse DSL source text into an AST
pub fn parse_dsl(source: &str) -> Result<Ast> {
    let pairs = DslParser::parse(Rule::program, source).map_err(|e| {
        let (line, col) = match e.line_col {
            pest::error::LineColLocation::Pos((l, c)) => (l, c),
            pest::error::LineColLocation::Span((l, c), _) => (l, c),
        };
        EngineError::parse_error(line, col, format!("Parse error: {}", e))
    })?;

    let mut statements = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                for inner in pair.into_inner() {
                    if inner.as_rule() == Rule::statement {
                        statements.push(parse_statement(inner)?);
                    }
                }
            }
            Rule::EOI => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }

    Ok(Ast::new(statements))
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::const_decl => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            let value = parse_number(parts.next().unwrap())?;
            Ok(Statement::ConstDecl { name, value })
        }
        Rule::symbol_decl => {
            let name = inner.into_inner().next().unwrap().as_str().to_string();
            Ok(Statement::SymbolDecl { name })
        }
        Rule::matrix_decl => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            let value = parse_matrix_literal(parts.next().unwrap())?;
            Ok(Statement::MatrixDecl { name, value })
        }
        Rule::function_def => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            let params = if let Some(param_list) = parts.peek() {
                if param_list.as_rule() == Rule::param_list {
                    parse_param_list(parts.next().unwrap())?
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };
            let body = Box::new(parse_expr(parts.next().unwrap())?);
            Ok(Statement::FunctionDef { name, params, body })
        }
        Rule::hamiltonian_def => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            let params = if let Some(param_list) = parts.peek() {
                if param_list.as_rule() == Rule::param_list {
                    parse_param_list(parts.next().unwrap())?
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };
            let expr = Box::new(parse_expr(parts.next().unwrap())?);
            Ok(Statement::HamiltonianDef { name, params, expr })
        }
        Rule::measurement_def => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            let spec = parse_measurement_spec(parts.next().unwrap())?;
            Ok(Statement::MeasurementDef { name, spec })
        }
        Rule::experiment => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            let body = parse_experiment_body(parts.next().unwrap())?;
            Ok(Statement::Experiment { name, body })
        }
        _ => Err(EngineError::parse_error(
            0,
            0,
            format!("Unexpected statement rule: {:?}", inner.as_rule()),
        )),
    }
}

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    match pair.as_rule() {
        Rule::expr => {
            let mut inner = pair.into_inner();
            let mut left = parse_term(inner.next().unwrap())?;

            while let Some(op_or_term) = inner.next() {
                let op = op_or_term;
                let right = parse_term(inner.next().unwrap())?;
                left = match op.as_rule() {
                    Rule::add_op => Expr::Add(Box::new(left), Box::new(right)),
                    Rule::sub_op => Expr::Sub(Box::new(left), Box::new(right)),
                    _ => unreachable!(),
                };
            }
            Ok(left)
        }
        _ => parse_term(pair),
    }
}

fn parse_term(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    if pair.as_rule() != Rule::term {
        return parse_factor(pair);
    }

    let mut inner = pair.into_inner();
    let mut left = parse_factor(inner.next().unwrap())?;

    while let Some(op_or_factor) = inner.next() {
        let op = op_or_factor;
        let right = parse_factor(inner.next().unwrap())?;
        left = match op.as_rule() {
            Rule::mul_op => Expr::Mul(Box::new(left), Box::new(right)),
            Rule::div_op => Expr::Div(Box::new(left), Box::new(right)),
            _ => unreachable!(),
        };
    }
    Ok(left)
}

fn parse_factor(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    if pair.as_rule() != Rule::factor {
        return parse_primary(pair);
    }

    let mut inner = pair.into_inner();
    let base = parse_primary(inner.next().unwrap())?;

    if let Some(pow_op) = inner.next() {
        if pow_op.as_rule() == Rule::pow_op {
            let exponent = parse_number(inner.next().unwrap())?;
            Ok(Expr::Pow(Box::new(base), Box::new(Expr::Number(exponent))))
        } else {
            Ok(base)
        }
    } else {
        Ok(base)
    }
}

fn parse_primary(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::number => Ok(Expr::Number(parse_number(inner)?)),
        Rule::identifier => Ok(Expr::Identifier(inner.as_str().to_string())),
        Rule::matrix_literal => Ok(Expr::Matrix(parse_matrix_literal(inner)?)),
        Rule::vector_literal => Ok(Expr::Vector(parse_vector_literal(inner)?)),
        Rule::expr => parse_expr(inner),
        Rule::builtin_function => parse_builtin_function(inner),
        _ => Err(EngineError::parse_error(
            0,
            0,
            format!("Unexpected primary rule: {:?}", inner.as_rule()),
        )),
    }
}

fn parse_builtin_function(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let inner = pair.into_inner().next().unwrap();
    let func_name = inner.as_str();

    let mut args = inner.into_inner();

    match func_name {
        "dagger" => Ok(Expr::Dagger(Box::new(parse_expr(args.next().unwrap())?))),
        "trace" => Ok(Expr::Trace(Box::new(parse_expr(args.next().unwrap())?))),
        "tensor" => {
            let left = parse_expr(args.next().unwrap())?;
            let right = parse_expr(args.next().unwrap())?;
            Ok(Expr::Tensor(Box::new(left), Box::new(right)))
        }
        "commutator" => {
            let left = parse_expr(args.next().unwrap())?;
            let right = parse_expr(args.next().unwrap())?;
            Ok(Expr::Commutator(Box::new(left), Box::new(right)))
        }
        "anticommutator" => {
            let left = parse_expr(args.next().unwrap())?;
            let right = parse_expr(args.next().unwrap())?;
            Ok(Expr::AntiCommutator(Box::new(left), Box::new(right)))
        }
        "expm" => Ok(Expr::Expm(Box::new(parse_expr(args.next().unwrap())?))),
        "sqrt" => Ok(Expr::Sqrt(Box::new(parse_expr(args.next().unwrap())?))),
        "sin" => Ok(Expr::Sin(Box::new(parse_expr(args.next().unwrap())?))),
        "cos" => Ok(Expr::Cos(Box::new(parse_expr(args.next().unwrap())?))),
        "exp" => Ok(Expr::Exp(Box::new(parse_expr(args.next().unwrap())?))),
        _ => Err(EngineError::parse_error(
            0,
            0,
            format!("Unknown builtin function: {}", func_name),
        )),
    }
}

fn parse_number(pair: pest::iterators::Pair<Rule>) -> Result<f64> {
    pair.as_str()
        .parse()
        .map_err(|_| EngineError::parse_error(0, 0, "Invalid number"))
}

fn parse_matrix_literal(pair: pest::iterators::Pair<Rule>) -> Result<MatrixLiteral> {
    let mut rows = Vec::new();
    for row_pair in pair.into_inner() {
        let mut row = Vec::new();
        for elem in row_pair.into_inner() {
            row.push(parse_expr(elem)?);
        }
        rows.push(row);
    }
    Ok(MatrixLiteral { rows })
}

fn parse_vector_literal(pair: pest::iterators::Pair<Rule>) -> Result<VectorLiteral> {
    let mut elements = Vec::new();
    for elem in pair.into_inner() {
        elements.push(parse_expr(elem)?);
    }
    Ok(VectorLiteral { elements })
}

fn parse_param_list(pair: pest::iterators::Pair<Rule>) -> Result<Vec<String>> {
    Ok(pair
        .into_inner()
        .map(|p| p.as_str().to_string())
        .collect())
}

fn parse_measurement_spec(pair: pest::iterators::Pair<Rule>) -> Result<MeasurementSpec> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::projective_measurement => {
            let mut projectors = Vec::new();
            for mat in inner.into_inner() {
                projectors.push(parse_matrix_literal(mat)?);
            }
            Ok(MeasurementSpec::Projective { projectors })
        }
        Rule::povm_measurement => {
            let mut effects = Vec::new();
            for mat in inner.into_inner() {
                effects.push(parse_matrix_literal(mat)?);
            }
            Ok(MeasurementSpec::POVM { effects })
        }
        _ => unreachable!(),
    }
}

fn parse_experiment_body(pair: pest::iterators::Pair<Rule>) -> Result<ExperimentBody> {
    let mut init = None;
    let mut evolution = None;
    let mut measurements = None;

    for stmt in pair.into_inner() {
        match stmt.as_rule() {
            Rule::init_statement => {
                init = Some(parse_state_spec(stmt.into_inner().next().unwrap())?);
            }
            Rule::evolution_statement => {
                evolution = Some(parse_evolution_spec(stmt.into_inner().next().unwrap())?);
            }
            Rule::measurement_schedule_statement => {
                measurements = Some(parse_measurement_schedule(
                    stmt.into_inner().next().unwrap(),
                )?);
            }
            _ => {}
        }
    }

    Ok(ExperimentBody {
        init,
        evolution,
        measurements,
    })
}

fn parse_state_spec(pair: pest::iterators::Pair<Rule>) -> Result<StateSpec> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::vector_literal => Ok(StateSpec::Ket(parse_vector_literal(inner)?)),
        Rule::matrix_literal => Ok(StateSpec::Rho(parse_matrix_literal(inner)?)),
        _ => unreachable!(),
    }
}

fn parse_evolution_spec(pair: pest::iterators::Pair<Rule>) -> Result<EvolutionSpec> {
    let mut parts = pair.into_inner();
    let state_name = parts.next().unwrap().as_str().to_string();
    let hamiltonian_name = parts.next().unwrap().as_str().to_string();
    let timegrid = parse_timegrid(parts.next().unwrap())?;

    let mut lindblad_ops = Vec::new();
    for lindblad in parts {
        lindblad_ops.push(parse_lindblad_term(lindblad)?);
    }

    Ok(EvolutionSpec {
        state_name,
        hamiltonian_name,
        timegrid,
        lindblad_ops,
    })
}

fn parse_timegrid(pair: pest::iterators::Pair<Rule>) -> Result<TimeGrid> {
    let inner = pair.into_inner().next().unwrap();
    let mut nums = inner.into_inner();

    if inner.as_str().starts_with("timegrid") {
        let t0 = parse_number(nums.next().unwrap())?;
        let dt = parse_number(nums.next().unwrap())?;
        let n_steps = parse_number(nums.next().unwrap())? as usize;
        Ok(TimeGrid::Regular { t0, dt, n_steps })
    } else {
        let times: Result<Vec<f64>> = nums.map(parse_number).collect();
        Ok(TimeGrid::Explicit { times: times? })
    }
}

fn parse_lindblad_term(pair: pest::iterators::Pair<Rule>) -> Result<LindbladTerm> {
    let mut parts = pair.into_inner();
    let operator_name = parts.next().unwrap().as_str().to_string();
    let rate = Box::new(parse_expr(parts.next().unwrap())?);
    Ok(LindbladTerm {
        operator_name,
        rate,
    })
}

fn parse_measurement_schedule(pair: pest::iterators::Pair<Rule>) -> Result<MeasurementSchedule> {
    let mut events = Vec::new();
    for event in pair.into_inner() {
        let mut parts = event.into_inner();
        let time = parse_number(parts.next().unwrap())?;
        let measurement_name = parts.next().unwrap().as_str().to_string();
        events.push(MeasurementEvent {
            time,
            measurement_name,
        });
    }
    Ok(MeasurementSchedule { events })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_const() {
        let source = "const omega = 1.0;";
        let ast = parse_dsl(source).unwrap();
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_matrix() {
        let source = "matrix sigma_x = [[0, 1], [1, 0]];";
        let ast = parse_dsl(source).unwrap();
        assert_eq!(ast.statements.len(), 1);
    }
}
