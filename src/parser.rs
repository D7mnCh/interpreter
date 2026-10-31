/*
program        → declaration* EOF ;
declaration    → VarDecl
               | statement
VarDecl        → "var" IDENTIFIAR  "=" expressoin ";"
statement      → exprStmt
               | printStmt ;
exprStmt       → expression ";" ;
printStmt      → print expression ";" ;
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" | IDENTIFIAR ;
*/

/*
TODO:
    - impl tests
    - report error in place if doen't need context
NOTE
    - error hanlding is kinda bad
    - how do i get a bool or nil ?
*/

use super::{
    ast::{BinaryOp, Expr, Stmt, UnaryOp},
    scanner::{Token, TokenType},
};

impl From<TokenType> for BinaryOp {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Plus => BinaryOp::Add,
            TokenType::Minus => BinaryOp::Sub,
            TokenType::Slash => BinaryOp::Div,
            TokenType::Asterisk => BinaryOp::Mul,
            TokenType::BangEqual => BinaryOp::Ne,
            TokenType::EqualEqual => BinaryOp::EqEq,
            TokenType::Less => BinaryOp::Lt,
            TokenType::LessEqual => BinaryOp::Le,
            TokenType::Greater => BinaryOp::Gt,
            TokenType::GreaterEqual => BinaryOp::Ge,
            TokenType::And => BinaryOp::And,
            TokenType::Or => BinaryOp::Or,
            _ => {
                panic!(
                    "caller should only convert to BinaryOp if TokenType is an operation variant"
                )
            }
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
    ExpectedSemicolon { found: Token },
    ExpectedExpr { found: Token },
    ExpectedToken { expected: TokenType, found: Token },
    ExpectedNum { found: Token },
    ExpectedIdentifier { found: Token },
    ExpectedEqualAfterIdentifier { found: Token },
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
                "[Parse Error] Line [{}:{}] at end",
                token.get_line(),
                token.get_lexeme_pos()
            )
        } else {
            format!(
                "[Parse Error] Line [{}:{}]",
                token.get_line(),
                token.get_lexeme_pos(),
            )
        }
    }

    fn report(&self) {
        match self {
            Self::ExpectedNum { found } => eprintln!(
                "{}: Expected a Number literal after expression, found \"{}\"",
                self.at_which_line(&found),
                found.get_lexeme()
            ),
            Self::ExpectedToken { expected, found } => eprintln!(
                "{}: Expected '{}' after expression, found '{}'",
                self.at_which_line(&found),
                expected.as_str(),
                found.get_lexeme()
            ),
            Self::ExpectedExpr { found } => eprintln!(
                "{} Expected expr, found \"{}\"",
                self.at_which_line(&found),
                found.get_lexeme()
            ),
            Self::ExpectedSemicolon { found } => eprintln!(
                "{} Expected ';', found \"{}\"",
                self.at_which_line(&found),
                found.get_lexeme()
            ),
            Self::ExpectedIdentifier { found } => eprintln!(
                "{} Expected \"identifier\", found \"{}\"",
                self.at_which_line(&found),
                found.get_lexeme()
            ),
            Self::ExpectedEqualAfterIdentifier { found } => eprintln!(
                "{} Expected \"=\" after identifier, found \"{}\"\
                NOTE: git it a value",
                self.at_which_line(&found),
                found.get_lexeme()
            ),
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
        // needed especially for REPL, e.g if missing ';' at "EOF", next_token() get called
        // at parse() for next tokens resulting out of bound indexing
        if !self.is_tokens_end() {
            self.current_token_indx += 1;
        }

        self.tokens[self.current_token_indx].clone()
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

    // TODO:
    // NOTE: need tracking
    fn pretty(&self) -> String {
        todo!()
    }
}

// the actuall parser impl using the recursion descenet method
impl Parser {
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_current_token_type(&[TokenType::Var]) {
            return self.variable_declaration();
        }

        self.statement()
    }

    fn variable_declaration(&mut self) -> Result<Stmt, ParseError> {
        if !self.match_current_token_type(&[TokenType::Identifier]) {
            return Err(ParseError::ExpectedIdentifier {
                found: self.peek_current_token(),
            });
        }

        let var_name = self.prev_token().get_lexeme().to_owned();
        if !self.match_current_token_type(&[TokenType::Equal]) {
            return Err(ParseError::ExpectedEqualAfterIdentifier {
                found: self.peek_current_token(),
            });
        }
        let expr = self.expression()?;
        if !self.match_current_token_type(&[TokenType::Semicolon]) {
            return Err(ParseError::ExpectedSemicolon {
                found: self.peek_current_token(),
            });
        }
        Ok(Stmt::var_decl(var_name, expr))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_current_token_type(&[TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        if !self.match_current_token_type(&[TokenType::Semicolon]) {
            return Err(ParseError::ExpectedSemicolon {
                found: self.peek_current_token(),
            });
        }

        Ok(Stmt::Expr(expr))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        if !self.match_current_token_type(&[TokenType::Semicolon]) {
            return Err(ParseError::ExpectedSemicolon {
                found: self.peek_current_token(),
            });
        }

        Ok(Stmt::Print(expr))
    }

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
            TokenType::And,
            TokenType::Or,
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

            expr = Ok(Expr::binary(left, op, right?));
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
            if !self.match_current_token_type(&[TokenType::RightParen]) {
                return Err(ParseError::ExpectedToken {
                    expected: TokenType::RightParen,
                    found: self.peek_current_token(),
                });
            }
            Ok(Expr::grouping(expr))
            // i did handle if trying to access of an undeclared variable in variable_declaration(), so no error handling here
        } else if self.match_current_token_type(&[TokenType::Identifier]) {
            let token = self.prev_token();
            Ok(Expr::ident(token))
        } else {
            Err(ParseError::ExpectedExpr {
                found: self.peek_current_token(),
            })
        };
    }

    // i was retuning Vec<Option<Stmt>> from self.statement(), there's no need for the None
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_tokens_end() {
            // i was doing match expr inside push() -.-. that's make the code less readable. match first then push
            match self.declaration().inspect_err(|err| err.report()) {
                Ok(statement) => statements.push(statement),
                Err(_) => {
                    self.next_token();
                }
            };
        }
        statements
    }
}
