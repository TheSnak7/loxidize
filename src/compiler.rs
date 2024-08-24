use logos::Logos;

use crate::{
    ast::Ast, bytecode::Bytecode, bytecode_compiler::BytecodeCompiler, parser::Parser, token::Token,
};

pub struct Compiler {}

impl Compiler {
    pub fn compile(&self, code: &str) -> Bytecode {
        println!("Started compiling");

        println!("Received: '{}'", code);

        let mut lex = Token::lexer(code);
        let mut parser = Parser::new(code, &mut lex);
        let ast = parser.parse_root().unwrap();
        println!("{:?}", ast);
        let bytecode_compiler = BytecodeCompiler::new(&ast);
        let bytecode = bytecode_compiler.compile();

        println!("{}", bytecode.disassemble("test"));

        bytecode
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self {}
    }
}
