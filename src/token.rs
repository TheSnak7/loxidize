use std::{num::ParseFloatError, string};

use logos::{Logos, Skip};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct TokenSpan {
    pub start: u32,
    pub end: u32,
}

impl TokenSpan {
    pub fn string<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start as usize..self.end as usize]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    #[default]
    InvalidNumber,
}

impl From<ParseFloatError> for LexingError {
    fn from(_: ParseFloatError) -> Self {
        LexingError::InvalidNumber
    }
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(error = LexingError)]
#[logos(extras = (usize, usize))]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    // Single character tokens
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,

    // One or two character tokens
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,

    // Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", identifier)]
    Identifier(TokenSpan),
    #[regex(r#""[^"]*""#, string)]
    String(TokenSpan),
    #[regex(r"\d+(\.\d+)?", number)]
    Number(f64),

    // Keywords
    #[token("and")]
    And,
    #[token("class")]
    Class,
    #[token("else")]
    Else,
    #[token("false")]
    False,
    #[token("for")]
    For,
    #[token("fun")]
    Fun,
    #[token("if")]
    If,
    #[token("nil")]
    Nil,
    #[token("or")]
    Or,
    #[token("print")]
    Print,
    #[token("return")]
    Return,
    #[token("super")]
    Super,
    #[token("var")]
    Var,
    #[token("while")]
    While,

    //For error reporting
    #[regex(r"\n", newline)]
    Newline,

    EOF,
}

fn identifier(lex: &mut logos::Lexer<Token>) -> (TokenSpan) {
    // Length of source code is guaranteed to be less than u32::MAX
    TokenSpan {
        start: lex.span().start as u32,
        end: lex.span().end as u32,
    }
}

fn string(lex: &mut logos::Lexer<Token>) -> (TokenSpan) {
    // Length of source code is guaranteed to be less than u32::MAX
    TokenSpan {
        start: lex.span().start as u32,
        end: lex.span().end as u32,
    }
}

fn number(lex: &mut logos::Lexer<Token>) -> Result<f64, LexingError> {
    lex.slice().parse().map_err(|_| LexingError::InvalidNumber)
}

fn newline(lex: &mut logos::Lexer<Token>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}
