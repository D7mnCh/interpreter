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
        - continue reading the book for now
        - remove unwrap
        - handle errors
*/

use crate::ast::{Expr, Op, UnaryOp};
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
            _ => panic!("caller should only convert to Op if token's is an operation variant"),
        }
    }
}

impl From<TokenType> for UnaryOp {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Minus => UnaryOp::Neg,
            TokenType::Bang => UnaryOp::Not,
            _ => panic!(
                "caller should only convert to UnaryOp if token's is an unary operation variant"
            ),
        }
    }
}

// parser utils
impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Self {
            tokens,
            current_token_indx: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        let token = self.tokens[self.current_token_indx].clone();
        self.current_token_indx += 1;

        token
    }

    fn match_current_token_type(&mut self, token_types: &[TokenType]) -> bool {
        if self.is_tokens_end() {
            return false;
        }

        let current_token_type = self.peek_current_token().get_token_type();

        if token_types
            .iter()
            .any(|token_type| Some(token_type) == current_token_type.as_ref())
        {
            self.next_token();
            return true;
        }

        false
    }

    fn peek_current_token(&self) -> Token {
        self.tokens[self.current_token_indx].clone()
    }

    fn prev_token(&self) -> Token {
        self.tokens[self.current_token_indx - 1].clone()
    }

    fn is_tokens_end(&self) -> bool {
        let token_type = self.peek_current_token().get_token_type();
        if token_type != Some(TokenType::Eof) {
            return false;
        }
        true
    }

    fn consume_if_match(&mut self) {}
}

// the actuall parser impl with the recursion decenet method
impl Parser {
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        // expr could be only left-side, or be a binary expr
        let mut expr = self.comparison();

        // (...)* maps to while loop
        while self.match_current_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let left = expr;
            let op = self.prev_token().get_token_type().unwrap().into();
            let right = self.comparison();

            expr = Expr::binary(right, op, left);
        }

        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_current_token_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let left = expr;
            let op = self.prev_token().get_token_type().unwrap().into();
            let right = self.term();

            expr = Expr::binary(right, op, left);
        }

        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_current_token_type(&[TokenType::Minus, TokenType::Plus]) {
            let left = expr;
            let op = self.prev_token().get_token_type().unwrap().into();
            let right = self.factor();

            expr = Expr::binary(right, op, left);
        }

        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_current_token_type(&[TokenType::Slash, TokenType::Asterisk]) {
            let left = expr;
            let op = self.prev_token().get_token_type().unwrap().into();
            let right = self.unary();

            expr = Expr::binary(right, op, left);
        }

        expr
    }
    fn unary(&mut self) -> Expr {
        // first match if it an unary op or a priamry (need a method?)
        if self.match_current_token_type(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary();
            let op: UnaryOp = self.prev_token().get_token_type().unwrap().into();
            return Expr::unary(op, right);
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        let token = self.prev_token();
        let literal = token.get_literal();
        return if self.match_current_token_type(&[TokenType::Number]) {
            Expr::num_liter(literal.parse().unwrap())
        } else if self.match_current_token_type(&[TokenType::StringLiter]) {
            Expr::string_liter(literal.to_owned())
        } else if self.match_current_token_type(&[TokenType::True]) {
            Expr::bool_liter(true)
        } else if self.match_current_token_type(&[TokenType::False]) {
            Expr::bool_liter(false)
        } else if self.match_current_token_type(&[TokenType::Nil]) {
            Expr::Nil
        } else if self.match_current_token_type(&[TokenType::LeftParen]) {
            let expr = self.expression();
            // consume next token, don't self.next_token(), check if it ")" and then consume it
            if !self.match_current_token_type(&[TokenType::RightParen]) {
                todo!("error");
            }
            Expr::grouping(expr)
        } else {
            todo!("error")
        };
    }
}
