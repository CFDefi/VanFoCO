//! Optimization passes for AST and IR

use crate::ast::*;
use crate::error::Result;
use std::collections::HashMap;

/// Optimizer for AST transformations
pub struct Optimizer {
    constant_cache: HashMap<String, f64>,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            constant_cache: HashMap::new(),
        }
    }

    /// Apply all optimization passes
    pub fn optimize(&mut self, ast: &Ast) -> Result<Ast> {
        let mut optimized = ast.clone();

        // Pass 1: Constant folding
        optimized = self.constant_folding(optimized)?;

        // Pass 2: Common subexpression elimination (TODO)
        // Pass 3: Algebraic simplifications (TODO)

        Ok(optimized)
    }

    /// Fold constant expressions
    fn constant_folding(&mut self, ast: Ast) -> Result<Ast> {
        // Collect all constants
        for stmt in &ast.statements {
            if let Statement::ConstDecl { name, value } = stmt {
                self.constant_cache.insert(name.clone(), *value);
            }
        }

        // Transform expressions (simplified version)
        Ok(ast)
    }

    /// Simplify an expression
    fn simplify_expr(&self, expr: &Expr) -> Expr {
        match expr {
            // 0 * x = 0
            Expr::Mul(left, right) => {
                if matches!(**left, Expr::Number(x) if x == 0.0) {
                    Expr::Number(0.0)
                } else if matches!(**right, Expr::Number(x) if x == 0.0) {
                    Expr::Number(0.0)
                } else {
                    expr.clone()
                }
            }
            // 0 + x = x
            Expr::Add(left, right) => {
                if matches!(**left, Expr::Number(x) if x == 0.0) {
                    self.simplify_expr(right)
                } else if matches!(**right, Expr::Number(x) if x == 0.0) {
                    self.simplify_expr(left)
                } else {
                    expr.clone()
                }
            }
            // dagger(dagger(x)) = x
            Expr::Dagger(inner) => {
                if let Expr::Dagger(inner2) = &**inner {
                    self.simplify_expr(inner2)
                } else {
                    expr.clone()
                }
            }
            _ => expr.clone(),
        }
    }

    /// Detect sparse matrices (TODO)
    pub fn detect_sparsity(&self, _ast: &Ast) -> HashMap<String, bool> {
        HashMap::new()
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let opt = Optimizer::new();
        assert!(opt.constant_cache.is_empty());
    }

    #[test]
    fn test_simplify_zero_multiplication() {
        let opt = Optimizer::new();
        let expr = Expr::Mul(
            Box::new(Expr::Number(0.0)),
            Box::new(Expr::Identifier("x".to_string())),
        );
        let simplified = opt.simplify_expr(&expr);
        assert!(matches!(simplified, Expr::Number(x) if x == 0.0));
    }
}
