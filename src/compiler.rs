use logos::Logos;

use crate::{ast::Ast, bytecode::Bytecode, parser::Parser, token::Token};

pub struct Compiler {}

impl Compiler {
    pub fn compile(&self, code: &str) -> Bytecode {
        println!("Started compiling");

        println!("Received: '{}'", code);

        let mut lex = Token::lexer("1\n");
        let mut parser = Parser::new(code, &mut lex);
        let ast = parser.parse_num_literal();

        println!("{:?}", ast);

        Bytecode::default()
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self {}
    }
}
