use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum ScanError {
    UnexpectedLexeme,
    UnterminatedString,
    ExpectedDigit,
}

impl ScanError {
    pub fn report(&self, line: usize) {
        match self {
            ScanError::UnexpectedLexeme => {
                eprintln!("[Scan Error]: Unexpected lexeme at line: {line}")
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
    // for source field, i tried using &str instead of &str, i face a problem where if lexemes is
    //compacted together like var=10, how you are going to tokenize it ?
    source_as_chars: &'a [char], // view
    tokens: Vec<Token>,
    start_lexeme_indx: usize, // depend on current_char_indx
    current_char_indx: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source_as_chars: &'a [char]) -> Self {
        Self {
            source_as_chars,
            tokens: Vec::new(),
            start_lexeme_indx: 0,
            current_char_indx: 0,
            line: 1,
        }
    }

    // NOTE don't like to have &mut self
    // &mut needed to mute self.current_char_indx
    fn scan_tokens(&mut self) -> Result<Option<TokenType>, ScanError> {
        let character = self.next_char();
        return match character {
            '(' => Ok(Some(TokenType::LeftParen)),
            ')' => Ok(Some(TokenType::RightParen)),
            //'{' => Ok(Some(TokenType::LeftBracet)),
            //'}' => Ok(Some(TokenType::RightBracet)),
            ',' => Ok(Some(TokenType::Comma)),
            '.' => Ok(Some(TokenType::Dot)),
            ';' => Ok(Some(TokenType::Semicolon)),
            '*' => Ok(Some(TokenType::Asterisk)),
            '-' => Ok(Some(TokenType::Minus)),
            '+' => Ok(Some(TokenType::Plus)),

            '=' => {
                if self.match_char('=') {
                    Ok(Some(TokenType::EqualEqual))
                } else {
                    Ok(Some(TokenType::Equal))
                }
            }
            '!' => {
                if self.match_char('=') {
                    Ok(Some(TokenType::BangEqual))
                } else {
                    Ok(Some(TokenType::Bang))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(Some(TokenType::GreaterEqual))
                } else {
                    Ok(Some(TokenType::Greater))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(Some(TokenType::LessEqual))
                } else {
                    Ok(Some(TokenType::Less))
                }
            }
            '"' => match self.handle_string_literal_scanning() {
                Ok(token_type) => Ok(Some(token_type)),
                Err(scan_error) => Err(scan_error),
            },
            '/' => {
                if !self.match_char('/') {
                    return Ok(Some(TokenType::Slash));
                }
                while self.peek_current() != '\n' {
                    let _ = self.next_char();
                }
                Ok(None)
            }
            ' ' | '\r' | '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                Ok(None)
            }

            // handle numbers and identifiers
            _ => {
                // handle numbers
                if self.peek_current().is_numeric() {
                    match self.handle_number_scanning() {
                        Ok(token_type) => Ok(Some(token_type)),
                        Err(scan_error) => Err(scan_error),
                    }
                } else if self.peek_current().is_alphabetic() {
                    match self.handle_identifiers() {
                        Ok(token_type) => Ok(Some(token_type)),
                        Err(scan_error) => Err(scan_error),
                    }
                } else {
                    Err(ScanError::UnexpectedLexeme)
                }
            }
        };
    }

    // NOTE didn't handle any scan errors
    fn handle_identifiers(&mut self) -> Result<TokenType, ScanError> {
        // whitespaces are not considered as alphabetic
        while self.peek_current().is_alphabetic() {
            let _ = self.next_char();
        }

        let lexeme: String = self.source_as_chars[self.start_lexeme_indx..self.current_char_indx]
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

    fn handle_number_scanning(&mut self) -> Result<TokenType, ScanError> {
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

    fn handle_string_literal_scanning(&mut self) -> Result<TokenType, ScanError> {
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

    fn get_literal(&self, token_type: &Option<TokenType>) -> String {
        let Some(token_type) = token_type else {
            return String::new();
        };
        match token_type {
            TokenType::StringLiter => {
                // trim that string first
                let begin_string = self.start_lexeme_indx + 1;
                let end_string = self.current_char_indx - 1;
                self.source_as_chars[begin_string..end_string]
                    .iter()
                    .collect::<String>()
                    .trim()
                    .to_string()
            }

            TokenType::Number => self.source_as_chars
                [self.start_lexeme_indx..self.current_char_indx]
                .iter()
                .collect(),

            _ => String::new(),
        }
    }

    // peek will not consume/increment current char/char index like next_char
    fn peek_current(&self) -> char {
        // NOTE if unwrap get error => out of bound index
        return *self.source_as_chars.get(self.current_char_indx).unwrap();
    }

    fn peek_next(&self) -> char {
        return *self
            .source_as_chars
            .get(self.current_char_indx + 1)
            .unwrap();
    }

    // the char is the current character before increament comumn_indx
    fn next_char(&mut self) -> char {
        let character = self.source_as_chars.get(self.current_char_indx).unwrap();
        self.current_char_indx += 1;

        *character
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        }

        let character = self.source_as_chars.get(self.current_char_indx).unwrap();
        if *character != expected {
            return false;
        }

        self.current_char_indx += 1;

        true
    }

    fn is_eof(&self) -> bool {
        self.current_char_indx >= self.source_as_chars.len()
    }

    // NOTE should have self.tokenize(&mut self) should push result of
    //self.scan_tokens(&self) and not the latter cuz used only for scan(no mutatioN)
    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_eof() {
            self.start_lexeme_indx = self.current_char_indx;
            // if token_type is None, that None can be valid tokens but redandant
            //(e.g commits, whitespaces), or it can be ScanError(e.g  @#$@#$)
            let token_type = match self.scan_tokens() {
                Ok(token_kind) => token_kind,
                Err(scan_error) => {
                    scan_error.report(self.line);
                    None
                }
            };
            let lexeme = self.source_as_chars[self.start_lexeme_indx..self.current_char_indx]
                .iter()
                .collect();
            let literal = self.get_literal(&token_type);

            let token = Token::new(token_type, lexeme, literal, self.line);
            self.tokens.push(token);
        }

        return self.tokens.clone(); // to visualize/debugging
    }
}

#[derive(Clone, Debug)]
// token == valid word(lexeme)
pub struct Token {
    pub token_type: Option<TokenType>,
    lexeme: String,
    literal: String, // NOTE i think stands for the value of the token if any
    line: usize,
}

impl Token {
    pub fn new(
        token_type: Option<TokenType>,
        lexeme: String,
        literal: String, // NOTE i think it should be generic type, not a String
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Clone, Debug)]
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
