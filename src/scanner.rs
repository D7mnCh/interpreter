#[derive(Clone, Debug)]
pub enum ScanErr {
    UnexpectedLexeme,
}

impl ScanErr {
    pub fn report(&self, line: usize) {
        match self {
            ScanErr::UnexpectedLexeme => eprintln!("Error: Unexpected lexeme at {line} line"),
            _ => todo!(),
        }
    }
}

pub struct Scanner {
    // TODO store source as Vec<char>
    source: Vec<char>,
    tokens: Vec<Result<Token, ScanErr>>,
    // start points to first char in start of the current lexeme
    start_lexeme_indx: usize,
    // current points to the current char of the current lexeme that being in scan
    current_char_indx: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start_lexeme_indx: 0,
            current_char_indx: 0,
            line: 1,
        }
    }

    /*
    TODO
        - scan for tokens
        - return Result<Token, ScanErr>, you will add token on tokenize() cuz it will take
        &mut self
        - take &self
    */
    fn scan_tokens(&self) -> Result<TokenType, ScanErr> {
        let source_as_chars = self.source.chars();
        let charac = source_as_chars.nth(self.current_char_indx);
        return match charac {
            Some('(') => Ok(TokenType::LeftParen),
            Some(')') => Ok(TokenType::RightParen),
            Some('{') => Ok(TokenType::LeftBracet),
            Some('}') => Ok(TokenType::RightBracet),
            Some(',') => Ok(TokenType::Comma),
            Some('.') => Ok(TokenType::Dot),
            Some(';') => Ok(TokenType::Semicolon),
            Some('*') => Ok(TokenType::Asterisk),
            Some('-') => Ok(TokenType::Minus),
            Some('+') => Ok(TokenType::Plus),
            // TODO check what is the next char
            Some('/') => Ok(TokenType::Slash),

            // check after current char to give a token for equal and negation
            //operators
            Some('=') => {
                if self.match_next_char('=') {
                    Ok(TokenType::EqualEqual)
                } else {
                    Ok(TokenType::Equal)
                }
            }
            Some('!') => Ok(TokenType::Bang),
            Some('>') => Ok(TokenType::Greater),
            Some('<') => Ok(TokenType::Less),
            Some('/') => todo!(),
            Some('\"') => {
                /*TODO
                    - ingore lexemes from current ` " ` entil the the next ` " `
                    - if didn't find the next ` " ` then return an error
                    - bring back whitespace inside that string
                */
                todo!()
            }

            _ => Err(ScanErr::UnexpectedLexeme),
        };
        //todo!()
    }

    fn is_at_end(&self) -> bool {
        self.current_char_indx >= self.source.chars().count()
    }

    // Note self.tokenize() should push result of self.scan_tokens()
    pub fn tokenize(&mut self) -> Vec<Result<Token, ScanErr>> {
        while !self.is_at_end() {
            self.start_lexeme_indx = self.current_char_indx;
            let token_type = self.scan_tokens().unwrap();
            // TODO match token_type

            let token = Token {
                token_type,
                lexeme: "".to_string(),
                literal: "".to_string(),
                line: self.line,
            };
            self.tokens.push(Ok(token));
        }

        self.tokens.push(Ok(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        }));

        return self.tokens.clone();
    }

    fn match_next_char(&self, next: char) -> bool {
        let source_as_chars = self.source.chars();
        // NOTE nth will take &mut, and i don't want that
        if source_as_chars.nth(self.current_char_indx) != Some(next) {
            return false;
        }
        true
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Self {
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
    LeftBracet,
    RightBracet,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Asterisk,

    // one or two character tokens.
    Bang, // !
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
    // NOTE turn Number to Int and Float ?
    Number,

    // keywords.
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

    Eof,
}
