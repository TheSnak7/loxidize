/*
Hand-written recursive descent parser for now.
In the future a generator could be used.
Note: Look into Zig's parser more.
*/

use std::fmt::Debug;

use crate::token::Token;

#[derive(Debug)]

pub struct Lit {
    pub kind: LitKind,
    // Number for now
    pub symbol: f64,
}

#[derive(Debug)]
pub enum LitKind {
    Number,
}

impl From<f64> for Lit {
    fn from(value: f64) -> Self {
        Self {
            kind: LitKind::Number,
            symbol: value,
        }
    }
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}

impl Expr {
    /*pub fn precedence(&self) -> ExprPrecedence {
        match &self.kind {
            ExprKind::Binary(op, ..) => ExprPrecedence::Binary(op),
            _ => unimplemented!("More operators to come"),
        }
    }*/
}

pub enum ExprPrecedence {
    Binary(BinOpKind),
}

impl ExprPrecedence {
    pub fn order(self) -> i8 {
        match self {
            ExprPrecedence::Binary(op) => AssocOp::from_ast_binop(op).precedence() as i8,
        }
    }
}

pub enum AssocOp {
    Add,
}

impl AssocOp {
    pub fn from_token(token: &Token) -> Option<AssocOp> {
        match token {
            Token::Plus => Some(AssocOp::Add),
            _ => unimplemented!("More operators to come"),
        }
    }

    pub fn from_ast_binop(op: BinOpKind) -> Self {
        match op {
            BinOpKind::Add => AssocOp::Add,
        }
    }

    pub fn to_ast_binop(&self) -> BinOpKind {
        match *self {
            AssocOp::Add => BinOpKind::Add,
        }
    }

    pub fn precedence(&self) -> usize {
        match *self {
            AssocOp::Add => 8,
        }
    }
}

pub type BinOp = BinOpKind;

#[derive(Debug)]
pub enum ExprKind {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Lit(Lit),
}

#[derive(Debug)]

pub enum BinOpKind {
    Add,
}

#[derive(Debug)]
pub struct Ast {
    pub root: Expr,
}
