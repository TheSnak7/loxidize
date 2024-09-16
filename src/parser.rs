use crate::{
    ast::{Ast, BinOpKind, Expr, ExprKind, Lit, Precedence},
    token::Token,
};

pub struct Parser<'a> {
    token: Token,
    prev_token: Token,
    prev_line: usize,
    prev_slice: &'a str,
    lexer: &'a mut logos::Lexer<'a, Token>,
    had_error: bool,
    at_end: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut logos::Lexer<'a, Token>) -> Self {
        let parser = Parser {
            // Placeholder
            token: Token::Bang,
            prev_token: Token::Bang,
            prev_line: 0,
            prev_slice: &"",
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
            match token.clone() {
                Ok(token) => {
                    self.token = token;
                }
                Err(e) => self.error(&format!("Lexing error: {:?}", e)),
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

    pub fn parse_root(&mut self) -> Result<Ast, ()> {
        // Set up initial state
        self.advance();
        self.advance();
        let expr = self.parse_expression(Precedence::None);
        let ast = Ast { root: expr };
        Ok(ast)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Expr {
        let mut left = self.parse_prefix();

        while precedence < Precedence::from_token(&self.prev_token) {
            left = self.parse_infix(left);
        }
        return left;
    }

    fn parse_prefix(&mut self) -> Expr {
        match self.prev_token {
            Token::Number(_) => self.parse_num_literal(),
            default => unimplemented!("Unimplemented: {:?}", default),
        }
    }

    fn parse_infix(&mut self, left: Expr) -> Expr {
        match self.prev_token {
            Token::Plus | Token::Minus => {
                //self.advance();
                self.parse_binop(left)
            }
            _ => unimplemented!(),
        }
    }

    fn parse_binop(&mut self, left: Expr) -> Expr {
        println!("Current operator token is: {:?}", self.prev_token);
        let lhs = Box::new(left);
        let op = match self.prev_token {
            Token::Plus => BinOpKind::Add,
            other => unimplemented!("Unimplemented Binops: {}", format!("{:?}", other)),
        };

        self.advance();
        let rhs = Box::new(self.parse_num_literal());
        let expr = Expr {
            kind: ExprKind::Binary(op, lhs, rhs),
        };
        expr
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
        let kind = ExprKind::Lit(literal);
        let expr = Expr { kind: kind };
        expr
    }
}
