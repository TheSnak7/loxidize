use crate::ast::{Expr, ExprKind};

pub trait VisitorResult {
    fn output() -> Self;
}

pub trait Visitor<'ast>: Sized {
    type Result: VisitorResult;

    fn visit_expr(&mut self, expr: &'ast Expr) -> Self::Result {
        walk_expr(self, expr)
    }

    fn visit_expr_post(&mut self, expr: &'ast Expr) -> Self::Result {
        Self::Result::output()
    }
}

pub fn walk_expr<'a, V: Visitor<'a>>(visitor: &mut V, expression: &'a Expr) -> V::Result {
    let Expr { kind } = expression;

    match kind {
        ExprKind::Binary(_op, lhs, rhs) => {
            visitor.visit_expr(&lhs);
            visitor.visit_expr(&rhs);
        }
        ExprKind::Lit(lit) => {
            unimplemented!("Lit not impl");
        }
    }

    visitor.visit_expr_post(expression)
}
