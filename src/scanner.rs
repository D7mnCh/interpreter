#[derive(Clone, Debug)]
pub enum ScanError {
    UnexpectedLexeme,
    FileEmpty,
}

impl ScanError {
    pub fn report(&self, line: usize) {
        match self {
            ScanError::UnexpectedLexeme => eprintln!("Error: Unexpected lexeme at {line} line"),
            _ => todo!(),
        }
    }
}

pub struct Scanner<'a> {
    // for source field, i tried using &str instead of &str, i face a problem where if lexemes is
    //compacted together like var=10, how you are going to tokenize it ?
    source_as_chars: &'a [char], // view
    tokens: Vec<Result<Token, ScanError>>,
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
    fn scan_tokens(&mut self) -> Result<TokenType, ScanError> {
        let character = self.next_char()?;
        return match character {
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            //'{' => Ok(TokenType::LeftBracet),
            //'}' => Ok(TokenType::RightBracet),
            ',' => Ok(TokenType::Comma),
            '.' => Ok(TokenType::Dot),
            ';' => Ok(TokenType::Semicolon),
            '*' => Ok(TokenType::Asterisk),
            '-' => Ok(TokenType::Minus),
            '+' => Ok(TokenType::Plus),

            '=' => {
                if self.match_char('=')? {
                    Ok(TokenType::EqualEqual)
                } else {
                    Ok(TokenType::Equal)
                }
            }
            '!' => Ok(TokenType::Bang),
            '>' => Ok(TokenType::Greater),
            '<' => Ok(TokenType::Less),
            //'/' => ,
            //'\"'=> ,
            _ => Err(ScanError::UnexpectedLexeme),
        };
    }

    // returning Result<char,E> where char is current character before increament comumn_indx
    fn next_char(&mut self) -> Result<char, ScanError> {
        let Some(character) = self.source_as_chars.get(self.current_char_indx) else {
            return Err(ScanError::FileEmpty);
        };
        self.current_char_indx += 1;

        Ok(*character)
    }

    fn match_char(&mut self, expected: char) -> Result<bool, ScanError> {
        if self.is_eof() {
            return Ok(false);
        }

        match self.source_as_chars.get(self.current_char_indx) {
            Some(character) => {
                if *character != expected {
                    return Ok(false);
                }
            }
            None => return Err(ScanError::FileEmpty),
        }

        self.current_char_indx += 1;

        Ok(true)
    }

    fn is_eof(&self) -> bool {
        self.current_char_indx >= self.source_as_chars.len()
    }

    // NOTE should have self.tokenize(&mut self) should push result of
    //self.scan_tokens(&self) and not the latter cuz used only for scan(no mutation)
    pub fn tokenize(&mut self) -> Vec<Result<Token, ScanError>> {
        while !self.is_eof() {
            self.start_lexeme_indx = self.current_char_indx;
            let token_type = self.scan_tokens().ok();
            let lexeme = self.source_as_chars[self.start_lexeme_indx..self.current_char_indx]
                .iter()
                .collect::<String>();

            let token = Token::new(token_type, lexeme, "".to_string(), self.line);
            self.tokens.push(Ok(token));
        }

        self.tokens.push(Ok(Token {
            token_type: Some(TokenType::Eof), // NOTE Eof is not a token
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        }));

        return self.tokens.clone(); // to visualize/debugging
    }
}

#[derive(Clone, Debug)]
// token == valid word
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

    Eof, // needed
}
