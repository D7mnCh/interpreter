/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
*/

/*
TODO
        - remove unwrap
        - handle errors
*/

use crate::ast::{Expr, Op};
use crate::scanner::{Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current_token_indx: usize,
}

impl From<TokenType> for Op {
    
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Plus => Op::Add,
            TokenType::Minus => Op::Sub,
            TokenType::Slash => Op::Div,
            TokenType::Asterisk => Op::Mul,
            TokenType::BangEqual => Op::Ne,
            TokenType::EqualEqual => Op::EqEq,
            TokenType::Less => Op::Lt,
            TokenType::LessEqual => Op::Le,
            TokenType::Greater => Op::Gt,
            TokenType::GreaterEqual => Op::Ge,
            _ => panic!("caller should only give an operator token"),
        }
    }
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Self {
            tokens,
            current_token_indx: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        // expr could be left-side or it could be binary expr
        let mut expr = self.comparison();

        while self.match_current_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let left = expr;
            let op = self.prev_token().get_token_type().unwrap().into();
            let right = self.comparison();
            expr = Expr::binary(right, op, left);
        }

        expr
    }

    fn next_token(&mut self) -> Token {
        let token = self.tokens[self.current_token_indx].clone();
        self.current_token_indx += 1;

        token
    }

    fn match_current_token_type(&mut self, expected_token_types: &[TokenType]) -> bool {
        for token_type in expected_token_types {
            if self.check_token_type(token_type) {
                self.next_token();
                return true;
            }
        }
        false
    }

    fn check_token_type(&self, expected_token_type: &TokenType) -> bool {
        if self.is_tokens_end() {
            return false;
        }
        self.peek_current_token().get_token_type().unwrap() == *expected_token_type
    }

    // used to check eof and to prevent panic from current_token_indx out of bound
    fn peek_current_token(&self) -> Token {
        self.tokens[self.current_token_indx].clone()
    }

    fn prev_token(&self) -> Token {
        self.tokens[self.current_token_indx - 1].clone()
    }

    fn is_tokens_end(&self) -> bool {
        let token_type = self.peek_current_token().get_token_type().unwrap();
        if token_type != TokenType::Eof {
            return false;
        }
        true
    }

    fn comparison(&mut self) -> Expr {
        todo!()
    }
    fn term(&mut self) -> Expr {
        todo!()
    }
    fn factor(&mut self) -> Expr {
        todo!()
    }
    fn unary(&mut self) -> Expr {
        todo!()
    }
    fn primary(&mut self) -> Expr {
        todo!()
    }
}
