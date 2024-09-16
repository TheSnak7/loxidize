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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Summation,
    Multiplication,
    Division,
}

impl Precedence {
    #[must_use]
    pub fn from_token(token: &Token) -> Precedence {
        match token {
            Token::EOF => Precedence::None,
            Token::Plus | Token::Minus => Precedence::Summation,
            Token::Star => Precedence::Multiplication,
            Token::Slash => Precedence::Division,
            default => unimplemented!("Illegal Token {:?}", default),
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
    Sub,
}

#[derive(Debug)]
pub struct Ast {
    pub root: Expr,
}
