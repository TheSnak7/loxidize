use logos::Logos;

use crate::{bytecode::Bytecode, token::Token};

pub struct Compiler {}

impl Compiler {
    pub fn compile(&self, code: &str) -> Bytecode {
        println!("Started compiling");

        println!("Received: '{}'", code);

        let mut lex = Token::lexer("1 + 1\n");

        assert_eq!(lex.next(), Some(Ok(Token::Number(1.0))));
        assert_eq!(lex.span(), 0..1);
        assert_eq!(lex.slice(), "1");

        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        assert_eq!(lex.span(), 2..3);
        assert_eq!(lex.slice(), "+");

        assert_eq!(lex.next(), Some(Ok(Token::Number(1.0))));
        assert_eq!(lex.span(), 4..5);
        assert_eq!(lex.slice(), "1");

        assert_eq!(lex.next(), None);

        println!("Finished lexing");

        Bytecode::default()
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self {}
    }
}
