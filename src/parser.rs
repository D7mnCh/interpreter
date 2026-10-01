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
        - use ParseError.report() of run funciton after using match ?
        - remove unwrap
        - handle errors
        - impl tests
*/

use crate::ast::{Expr, Op, UnaryOp};
use crate::scanner::{Token, TokenType};

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

enum ParseError {
    ExpectedExpr { found: Token },
    ExpectedToken { expected: TokenType, found: Token },
    ExpectedNum { found: Token },
}

// i think you can map the error to ParseError
// impl From<ParseFloatError> for ParseError {
//     fn from(_error: ParseFloatError) -> Self {
//         // eprintln!("{_error}");
//         Self::ExpectedNum
//     }
// }

impl ParseError {
    fn at_which_line(&self, token: &Token) -> String {
        let token_type = token.get_token_type();
        if token_type == TokenType::Eof {
            format!(
                "[Parse Error] Line {}:{} at end:",
                token.get_line(),
                token.get_char()
            )
        } else {
            format!(
                "[Parse Error] Line {}:{} at {}:",
                token.get_line(),
                token.get_char(),
                token.get_lexeme()
            )
        }
    }

    fn report(&self) {
        match self {
            Self::ExpectedNum { found } => eprintln!(
                "{}: Expected a Number literal after expression, found {}",
                self.at_which_line(&found),
                found.get_lexeme()
            ),
            Self::ExpectedToken { expected, found } => eprintln!(
                "{}: Expected '{:?} after expression, found {}' ",
                self.at_which_line(&found),
                // TODO, impl TokenType::as_str()
                expected,
                found.get_lexeme()
            ),
            Self::ExpectedExpr { found } => {
                eprintln!(
                    "{} Expected expr, found \"{}\"",
                    self.at_which_line(&found),
                    found.get_lexeme()
                )
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    parse_errors: Vec<ParseError>,
    current_token_indx: usize,
}

// parser utils
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Self {
            tokens,
            parse_errors: Vec::new(),
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
            .any(|token_type| token_type == &current_token_type)
        {
            self.next_token();
            return true;
        }

        false
    }

    fn peek_current_token(&self) -> Token {
        self.tokens[self.current_token_indx].clone()
    }

    // used after self.match_current_token_type()
    fn prev_token(&self) -> Token {
        self.tokens[self.current_token_indx - 1].clone()
    }

    fn is_tokens_end(&self) -> bool {
        let token_type = self.peek_current_token().get_token_type();
        if token_type != TokenType::Eof {
            return false;
        }
        true
    }

    fn consume_if_match(&mut self, token_type: TokenType) -> Option<ParseError> {
        if !self.match_current_token_type(&[token_type.clone()]) {
            return Some(ParseError::ExpectedToken {
                expected: token_type,
                found: self.peek_current_token(),
            });
        }
        None
    }
}

// the actuall parser impl using the recursion descenet method
impl Parser {
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        // expr could be only left-side, or be a binary expr
        let mut expr = self.comparison();

        // (...)* maps to while loop
        while self.match_current_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let left = expr?;
            let op = self.prev_token().get_token_type().into();
            let right = self.comparison()?;

            expr = Ok(Expr::binary(left, op, right));
        }

        expr
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();

        while self.match_current_token_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let left = expr?;
            let op = self.prev_token().get_token_type().into();
            let right = self.term()?;

            expr = Ok(Expr::binary(left, op, right));
        }

        expr
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();

        while self.match_current_token_type(&[TokenType::Minus, TokenType::Plus]) {
            let left = expr?;
            let op = self.prev_token().get_token_type().into();
            let right = self.factor()?;

            expr = Ok(Expr::binary(left, op, right));
        }

        expr
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();

        while self.match_current_token_type(&[TokenType::Slash, TokenType::Asterisk]) {
            let left = expr?;
            let op = self.prev_token().get_token_type().into();
            let right = self.unary();

            expr = Ok(Expr::binary(right?, op, left));
        }

        expr
    }
    fn unary(&mut self) -> Result<Expr, ParseError> {
        // first match if it an unary op or a priamry (need a method?)
        if self.match_current_token_type(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary()?;
            let op: UnaryOp = self.prev_token().get_token_type().into();

            return Ok(Expr::unary(op, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        return if self.match_current_token_type(&[TokenType::Number]) {
            let token = self.prev_token();
            let literal = token.get_literal();
            Ok(Expr::num_liter(literal.parse().unwrap()))
        } else if self.match_current_token_type(&[TokenType::StringLiter]) {
            let token = self.prev_token();
            let literal = token.get_literal();
            Ok(Expr::string_liter(literal.to_owned()))
        } else if self.match_current_token_type(&[TokenType::True]) {
            Ok(Expr::bool_liter(true))
        } else if self.match_current_token_type(&[TokenType::False]) {
            Ok(Expr::bool_liter(false))
        } else if self.match_current_token_type(&[TokenType::Nil]) {
            Ok(Expr::Nil)
        } else if self.match_current_token_type(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            // consume next token, don't self.next_token(), check if it ")" and then consume it
            if let Some(e) = self.consume_if_match(TokenType::RightParen) {
                return Err(e);
            }
            Ok(Expr::grouping(expr))
        } else {
            Err(ParseError::ExpectedExpr {
                found: self.peek_current_token(),
            })
        };
    }

    // for now parses only one expr
    // if self.expression() return an error, i should not have an Expr but None
    pub fn parse(&mut self) -> Option<Expr> {
        return match self.expression() {
            Ok(expr) => Some(expr),
            Err(err) => {
                err.report();
                None
            }
        };
    }
}
