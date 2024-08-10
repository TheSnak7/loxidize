use std::{num::ParseFloatError, process::id};

use logos::Logos;

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

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
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
    Identifier(String),
    #[regex(r#""[^"]*""#, string)]
    String(String),
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
}

fn identifier(lex: &mut logos::Lexer<Token>) -> String {
    lex.slice().to_string()
}

fn string(lex: &mut logos::Lexer<Token>) -> String {
    lex.slice()[1..lex.slice().len() - 1].to_string()
}

fn number(lex: &mut logos::Lexer<Token>) -> Result<f64, LexingError> {
    lex.slice().parse().map_err(|_| LexingError::InvalidNumber)
}
