use crate::token::{Token, TokenType, LiteralValue};

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Unary(TokenType, Box<Expr>),
    Binary(Box<Expr>, TokenType, Box<Expr>),
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();

        while matches!(self.peek().token_type, TokenType::PLUS | TokenType::MINUS) {
            let op = self.advance().token_type.clone();
            let right = self.parse_term();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }

        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_unary();

        while matches!(self.peek().token_type, TokenType::STAR | TokenType::SLASH | TokenType::PERCENT) {
            let op = self.advance().token_type.clone();
            let right = self.parse_unary();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }

        left
    }

    fn parse_unary(&mut self) -> Expr {
        if matches!(self.peek().token_type, TokenType::MINUS) {
            let op = self.advance().token_type.clone();
            let operand = self.parse_unary();
            return Expr::Unary(op, Box::new(operand));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        let token = self.advance();

        match token.token_type {
            TokenType::INTEGER => {
                let v = match &token.value {
                    LiteralValue::Integer(i) => *i,
                    _ => panic!("Expected integer literal"),
                };
                Expr::Integer(v)
            }

            TokenType::FLOAT => {
                let v = match &token.value {
                    LiteralValue::Float(f) => *f,
                    _ => panic!("Expected float literal"),
                };
                Expr::Float(v)
            }

            TokenType::LPAREN => {
                let expr = self.parse_expr();
                let closing = self.advance();
                if closing.token_type != TokenType::RPAREN {
                    panic!("Expected ')'");
                }
                expr
            }

            _ => panic!("Unexpected token '{:?}'", token.token_type),
        }
    }

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token<'a> {
        let token = &self.tokens[self.pos];
        if token.token_type != TokenType::END_OF_FILE {
            self.pos += 1;
        }
        token
    }
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Integer(n) => *n as f64,

            Expr::Float(n) => *n,

            Expr::Unary(op, operand) => {
                let r = operand.eval();
                match op {
                    TokenType::MINUS => -r,
                    TokenType::PLUS => r,
                    _ => unreachable!(),
                }
            }

            Expr::Binary(left, op, right) => {
                let l = left.eval();
                let r = right.eval();

                match op {
                    TokenType::PLUS => l + r,
                    TokenType::MINUS => l - r,
                    TokenType::STAR => l * r,
                    TokenType::SLASH => l / r,
                    TokenType::PERCENT => l % r,
                    _ => unreachable!(),
                }
            }
        }
    }
}