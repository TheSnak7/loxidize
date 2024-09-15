use crate::{
    ast::{AssocOp, Ast, BinOpKind, Expr, ExprKind, Lit},
    token::Token,
};

pub struct Parser<'a> {
    token: Token,
    prev_token: Token,
    prev_line: usize,
    prev_slice: &'a str,
    source: &'a str,
    lexer: &'a mut logos::Lexer<'a, Token>,
    had_error: bool,
    at_end: bool,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str, lexer: &'a mut logos::Lexer<'a, Token>) -> Self {
        let parser = Parser {
            // Placeholder
            token: Token::Bang,
            prev_token: Token::Bang,
            prev_line: 0,
            prev_slice: &"",
            source: src,
            lexer: lexer,
            had_error: false,
            at_end: false,
        };
        parser
    }

    pub fn expect(&mut self, token: Token) {}

    pub fn check(&mut self, token: Token) -> bool {
        false
    }

    pub fn eat(&mut self, token: Token) -> bool {
        let is_present = self.check(token);
        if is_present {
            self.advance()
        }
        is_present
    }

    pub fn advance(&mut self) {
        self.prev_line = self.lexer.extras.0;
        self.prev_slice = self.lexer.slice();
        self.prev_token = self.token;
        if let Some(token) = self.lexer.next() {
            if let Ok(token) = token {
                self.token = token;
            } else {
                self.error("Lexing error")
            }
        } else {
            self.token = Token::EOF
        }
    }

    pub fn error(&mut self, message: &str) {
        self.error_at(self.prev_line, self.prev_slice, message);
    }

    pub fn error_at_current(&mut self, message: &str) {
        self.error_at(self.lexer.extras.0, self.lexer.slice(), message);
    }

    pub fn error_at(&mut self, line: usize, lexeme: &str, message: &str) {
        eprint!("[line {}] Error", line);

        if self.at_end {
            eprint!(" at end");
        } else {
            eprint!(" at {}", lexeme)
        }
        eprintln!(": {message}");
        self.had_error = true;
    }

    pub fn get_parse_rule(token: &Token) -> Box<dyn Fn() -> ()> {
        unimplemented!()
    }

    pub fn parse_root(&mut self) -> Result<Ast, ()> {
        // Set up initial state
        self.advance();
        self.advance();
        let expr = self.parse_binop();
        let ast = Ast { root: expr };
        Ok(ast)
    }

    fn parse_binop(&mut self) -> Expr {
        let lhs = Box::new(self.parse_num_literal());
        let assoc_op = AssocOp::from_token(&self.prev_token);
        let op = match self.prev_token {
            Token::Plus => BinOpKind::Add,
            other => unimplemented!("Unimplemented Binops: {}", format!("{:?}", other)),
        };
        self.advance();
        let rhs = Box::new(self.parse_num_literal());
        Expr {
            kind: ExprKind::Binary(op, lhs, rhs),
        }
    }

    pub fn parse_num_literal(&mut self) -> Expr {
        let num = match &self.prev_token {
            Token::Number(num) => num.clone(),
            _ => panic!(
                "Unexpected token instead of number: {}",
                format!("{:?}", self.token)
            ),
        };
        self.advance();
        let literal = Lit::from(num);
        println!("Parsed: {:?}", literal);
        let kind = ExprKind::Lit(literal);
        let expr = Expr { kind: kind };
        expr
    }
}
