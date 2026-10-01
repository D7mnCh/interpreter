/*
- TODO need rewrite to make i more idiomatic rust(using rust std/features)
- https://github.com/vladmonea/crusty_interpreter/blob/main/docs/Project5_Parsing.md
- https://github.com/vladmonea/crusty_interpreter/blob/main/lux/src/tokenize.rs
*/

use std::collections::HashMap;

#[derive(Clone, Debug)]
// TODO give variants values, maybe fields  i
pub enum ScanError {
    UnexpectedLexeme(char),
    UnterminatedString,
    ExpectedDigit,
}

impl ScanError {
    pub fn report(&self, line: usize, column: usize) {
        match self {
            ScanError::UnexpectedLexeme(c) => {
                eprintln!("[Scan Error]: Unexpected lexeme \"{c:?}\" at {line}:{column}")
            }
            ScanError::UnterminatedString => {
                eprintln!("[Scan Error]: Unterminated string")
            }
            ScanError::ExpectedDigit => {
                eprintln!("[Scan Error]: Expected digit")
            }
        }
    }
}

pub struct Scanner<'a> {
    // &'a [char] is better then &'a str, former will allow working with utf8 chars,
    //and can iter without out of the box (cuz it's a slice)
    source: &'a [char], // view
    tokens: Vec<Token>,
    // depend on current_char_indx
    start_lexeme_indx: usize,
    current_char_indx: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [char]) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start_lexeme_indx: 0,
            current_char_indx: 0,
            line: 1,
        }
    }

    // &mut needed to mute self.current_char_indx
    fn scan_tokens(&mut self) -> Option<Result<TokenType, ScanError>> {
        //i was scanning source with lines() method, i face a problem where if the lexeme is
        //compacted together like var=10, i've to impl the current approach and i think it make the
        //code a bit noisy, solution is to scan char by char without using lines()
        let character = self.next_char();
        return match character {
            '(' => Some(Ok(TokenType::LeftParen)),
            ')' => Some(Ok(TokenType::RightParen)),
            //'{' => Some(Ok(TokenType::LeftBracet)),
            //'}' => Some(Ok(TokenType::RightBracet)),
            ',' => Some(Ok(TokenType::Comma)),
            '.' => Some(Ok(TokenType::Dot)),
            ';' => Some(Ok(TokenType::Semicolon)),
            '*' => Some(Ok(TokenType::Asterisk)),
            '-' => Some(Ok(TokenType::Minus)),
            '+' => Some(Ok(TokenType::Plus)),

            '=' => {
                if self.current_char_match('=') {
                    Some(Ok(TokenType::EqualEqual))
                } else {
                    Some(Ok(TokenType::Equal))
                }
            }
            '!' => {
                if self.current_char_match('=') {
                    Some(Ok(TokenType::BangEqual))
                } else {
                    Some(Ok(TokenType::Bang))
                }
            }
            '>' => {
                if self.current_char_match('=') {
                    Some(Ok(TokenType::GreaterEqual))
                } else {
                    Some(Ok(TokenType::Greater))
                }
            }
            '<' => {
                if self.current_char_match('=') {
                    Some(Ok(TokenType::LessEqual))
                } else {
                    Some(Ok(TokenType::Less))
                }
            }
            '"' => Some(self.scan_string()),
            '/' => {
                // handle one line comments
                if self.current_char_match('/') {
                    // didn't consume \n, consumed later after next call of scan_token()
                    while self.peek_current() != '\n' {
                        let _ = self.next_char();
                    }
                    // handle multi line comments
                } else if self.current_char_match('*') {
                    while self.peek_current() != '*' && self.peek_next() != '/' {
                        if self.peek_current() == '\n' {
                            self.line += 1;
                        }
                        let _ = self.next_char();
                    }
                    // consume '*' and '/'
                    let _ = self.next_char();
                    let _ = self.next_char();
                } else {
                    return Some(Ok(TokenType::Slash));
                }

                None
            }
            '\n' => {
                self.line += 1;
                None
            }
            c if c.is_whitespace() => None,
            // handle numbers and identifiers
            c => {
                // handle numbers
                // i was doing self.peek_current().is_numeric instead of 'c', but i do forget that
                //i increamnt current_char_indx at the begging of the function
                if c.is_numeric() {
                    return Some(self.scan_num());
                }

                // handle identifiers
                if c.is_alphabetic() || c == '_' {
                    return Some(self.scan_ident());
                }

                Some(Err(ScanError::UnexpectedLexeme(self.peek_current())))
            }
        };
    }

    // NOTE didn't handle any scan errors
    fn scan_ident(&mut self) -> Result<TokenType, ScanError> {
        // whitespaces are not considered as alphabetic
        while self.peek_current().is_alphabetic()
            || self.peek_current() == '_'
            || self.peek_current().is_numeric()
        {
            let _ = self.next_char();
        }

        let lexeme: String = self.source[self.start_lexeme_indx..self.current_char_indx]
            .iter()
            .collect();

        let keywords: HashMap<String, TokenType> = HashMap::from([
            ("then".to_string(), TokenType::Then),
            ("end".to_string(), TokenType::End),
            ("do".to_string(), TokenType::Do),
            ("and".to_string(), TokenType::And),
            ("or".to_string(), TokenType::Or),
            ("struct".to_string(), TokenType::Struct),
            ("if".to_string(), TokenType::If),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("True".to_string(), TokenType::True),
            ("func".to_string(), TokenType::Func),
            ("for".to_string(), TokenType::For),
            ("nil".to_string(), TokenType::Nil),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("self".to_string(), TokenType::SelfKeyword),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]);

        let Some(keyword) = keywords.get(&lexeme) else {
            return Ok(TokenType::Identifier);
        };

        Ok(keyword.to_owned())
    }

    fn scan_num(&mut self) -> Result<TokenType, ScanError> {
        while self.peek_current().is_numeric() {
            if self.peek_next().is_alphabetic() {
                return Err(ScanError::ExpectedDigit);
            }
            let _ = self.next_char();
        }

        if self.peek_current() == '.' && self.peek_next().is_numeric() {
            // consume the '.' in order the while loop under to work
            let _ = self.next_char();

            while self.peek_current().is_numeric() {
                if self.peek_next().is_alphabetic() {
                    return Err(ScanError::ExpectedDigit);
                }
                let _ = self.next_char();
            }
        }

        return Ok(TokenType::Number);
    }

    fn scan_string(&mut self) -> Result<TokenType, ScanError> {
        while self.peek_current() != '"' {
            // support for multiline
            if self.peek_current() == '\n' {
                self.line += 1;
            }
            let _ = self.next_char();
        }

        if self.is_eof() {
            return Err(ScanError::UnterminatedString);
        }

        // consume the closing '\"'
        let _ = self.next_char();

        Ok(TokenType::StringLiter)
    }

    fn get_literal(&self, token_type: &TokenType) -> String {
        match token_type {
            TokenType::StringLiter => {
                // trim that string first(remove both ")
                let begin_string = self.start_lexeme_indx + 1;
                let end_string = self.current_char_indx - 1;
                self.source[begin_string..end_string]
                    .iter()
                    .collect::<String>()
                    .trim()
                    .to_string()
            }

            TokenType::Number => self.source[self.start_lexeme_indx..self.current_char_indx]
                .iter()
                .collect(),

            _ => String::new(),
        }
    }

    // peek will not consume/increment current char/char index like next_char
    fn peek_current(&self) -> char {
        // NOTE if unwrap get error => out of bound index
        return *self.source.get(self.current_char_indx).unwrap();
    }

    fn peek_next(&self) -> char {
        return *self.source.get(self.current_char_indx + 1).unwrap();
    }

    // the returned char is the current character before increament current_char_indx
    fn next_char(&mut self) -> char {
        let character = self.source.get(self.current_char_indx).unwrap();
        self.current_char_indx += 1;

        *character
    }

    fn current_char_match(&mut self, expected: char) -> bool {
        // if didn't handle, i'll get out of index
        if self.is_eof() {
            return false;
        }

        let character = self.source.get(self.current_char_indx).unwrap();
        if *character != expected {
            return false;
        }

        self.current_char_indx += 1;

        true
    }

    // used to check eof and to prevent panic current_char_indx out of bound
    fn is_eof(&self) -> bool {
        self.current_char_indx >= self.source.len()
    }

    // NOTE should have self.tokenize(&mut self) should push result of
    //self.scan_tokens(&self) and not the latter cuz used only for scan(no mutatioN)
    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_eof() {
            self.start_lexeme_indx = self.current_char_indx;
            let Some(scan_result) = self.scan_tokens() else {
                continue;
            };
            let token_type = match scan_result {
                Ok(token_type) => token_type,
                Err(scan_error) => {
                    scan_error.report(self.line, self.current_char_indx + 1);
                    continue;
                }
            };

            let lexeme = self.source[self.start_lexeme_indx..self.current_char_indx]
                .iter()
                .collect();
            let literal = self.get_literal(&token_type);

            let token = Token::new(
                token_type,
                lexeme,
                literal,
                self.line,
                self.current_char_indx + 1,
            );
            self.tokens.push(token);
        }

        let token = Token::new(
            TokenType::Eof,
            "".to_owned(),
            "".to_owned(),
            self.line,
            self.current_char_indx,
        );
        self.tokens.push(token);

        return self.tokens.clone(); // to visualize/debugging
    }
}

#[derive(Clone, Debug, PartialEq)]
// token == valid word/lexeme
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
    current_char: usize,
}

impl Token {
    pub fn new(
        // it was set Option<TokenType> as a type, with it i could have None TokenType,
        //i could just continue or do early return when face None situation
        token_type: TokenType,
        lexeme: String,
        literal: String, // NOTE i think it should be generic type, not a String
        line: usize,
        current_char: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            current_char,
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn get_lexeme(&self) -> &String {
        &self.lexeme
    }

    pub fn get_literal(&self) -> &String {
        &self.literal
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_char(&self) -> usize {
        self.current_char
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Asterisk,
    Eof, // needed in parser

    // one or two character tokens.
    Bang, // -> !
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier,
    StringLiter,
    // NOTE turn Number into an Int and Float ?
    Number,

    // keywords.
    Func,
    Then,
    End,
    Do,
    And,
    Or,
    Struct,
    If,
    Else,
    False,
    True,
    For,
    Nil,
    Print,
    Return,
    SelfKeyword,
    Var,
    While,
}
