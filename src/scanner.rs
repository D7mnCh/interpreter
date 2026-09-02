#[derive(Clone, Debug)]
pub enum ScanError {
    UnexpectedLexeme,
}

impl ScanError {
    pub fn report(&self, line: usize) {
        match self {
            ScanError::UnexpectedLexeme => {
                eprintln!("[Scan Error]: Unexpected lexeme at line: {line}")
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
            _ => Err(ScanError::UnexpectedLexeme),
        };
    }

    // peek will not consume/increment current char/char index like next_char
    fn peek_current(&self) -> char {
        // NOTE if unwrap get error => out of bound index
        return *self.source_as_chars.get(self.current_char_indx).unwrap();
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
    //self.scan_tokens(&self) and not the latter cuz used only for scan(no mutation)
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
                .collect::<String>();

            let token = Token::new(token_type, lexeme, "".to_string(), self.line);
            self.tokens.push(token);
        }

        return self.tokens.clone(); // to visualize/debugging
    }
}

#[derive(Clone, Debug)]
// token == valid word(lexeme)
pub struct Token {
    token_type: Option<TokenType>,
    lexeme: String,
    literal: String, // NOTE i think stands for the value of the token if any
    line: usize,
}

impl Token {
    pub fn new(
        token_type: Option<TokenType>,
        lexeme: String,
        literal: String,
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
    Then,
    End,
    Do,
    And,
    Struct,
    Else,
    False,
    Func,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    SelfKeyword,
    True,
    Var,
    While,
}
