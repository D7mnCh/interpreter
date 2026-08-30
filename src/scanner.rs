#[derive(Clone, Debug)]
pub enum ScanErr {
    UnexpectedLexeme,
    FileEmpty,
}

impl ScanErr {
    pub fn report(&self, line: usize) {
        match self {
            ScanErr::UnexpectedLexeme => eprintln!("Error: Unexpected lexeme at {line} line"),
            _ => todo!(),
        }
    }
}

pub struct Scanner<'a> {
    // for source field, i tried using &str instead of &str, i face a problem where if lexemes is
    //compacted together like var=10, how you are going to tokenize it ?
    source_as_chars: &'a [char], // view
    tokens: Vec<Result<Token, ScanErr>>,
    start_lexeme_indx: usize, // depend on column_indx
    column_indx: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source_as_chars: &'a [char]) -> Self {
        Self {
            source_as_chars,
            tokens: Vec::new(),
            start_lexeme_indx: 0,
            column_indx: 0,
            line: 1,
        }
    }

    // NOTE don't like to have &mut self
    fn scan_tokens(&mut self) -> Result<TokenType, ScanErr> {
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
            _ => Err(ScanErr::UnexpectedLexeme),
        };
    }

    fn is_eof(&self) -> bool {
        self.column_indx >= self.source_as_chars.len() // how this works ?
    }

    // NOTE should have self.tokenize(&mut self) should push result of
    //self.scan_tokens(&self) and not the latter cuz used only for scan(no mutation)
    pub fn tokenize(&mut self) -> Vec<Result<Token, ScanErr>> {
        while !self.is_eof() {
            self.start_lexeme_indx = self.column_indx;
            let token_type = self.scan_tokens().unwrap();
            // TODO match token_type to handle errors(used unwrap()above)

            let token = Token::new(token_type, "".to_string(), "".to_string(), self.line);
            self.tokens.push(Ok(token));

            //self.advance_scanning_by_char();
        }

        self.tokens.push(Ok(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        }));

        return self.tokens.clone(); // to visualize/debugging
    }

    // returning Result<char,E> where char is current character before increament comumn_indx
    fn next_char(&mut self) -> Result<char, ScanErr> {
        let Some(character) = self.source_as_chars.get(self.column_indx) else {
            return Err(ScanErr::FileEmpty);
        };
        self.column_indx += 1;

        Ok(*character)
    }

    fn match_char(&mut self, expected: char) -> Result<bool, ScanErr> {
        if self.is_eof() {
            return Ok(false);
        }

        match self.source_as_chars.get(self.column_indx) {
            Some(character) => {
                if *character != expected {
                    return Ok(false);
                }
            }
            None => return Err(ScanErr::FileEmpty),
        }

        self.column_indx += 1;

        Ok(true)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String, // NOTE i think stands for the value of the token if any
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
