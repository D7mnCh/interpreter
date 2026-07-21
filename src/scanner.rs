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
    source: String,
    // can be None when empty source, also when trying to construct Scanner
    tokens: Vec<Result<Token, ScanErr>>,
    // start points to first char in start of the current lexeme
    start: usize,
    // current points to the current char of the current lexeme that being in scan
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /*
    TODO
        - on every line, scan for tokens
        - return Result<Token, ScanErr>
        - should not take &self
    */
    fn scan_tokens(&self) -> Result<Token, ScanErr> {
        todo!()
    }

    // TODO self.tokenize() should push result of self.scan_tokens()
    pub fn tokenize(&mut self) -> Vec<Result<Token, ScanErr>> {
        let source_lines = self.source.lines();

        for (line_num, line) in source_lines.enumerate() {
            //println!("found a line to iterate over chars");
            while line.chars().next() != None {
                let token_type = match line.chars().next() {
                    Some('(') => TokenType::LeftParen,
                    Some(')') => TokenType::RightParen,
                    Some('{') => TokenType::LeftBracet,
                    Some('}') => TokenType::RightBracet,
                    Some(',') => TokenType::Comma,
                    Some('.') => TokenType::Dot,
                    Some(';') => TokenType::Semicolon,
                    Some('*') => TokenType::Asterisk,
                    Some('-') => TokenType::Minus,
                    Some('+') => TokenType::Plus,
                    // TODO check what the next char
                    Some('/') => TokenType::Slash,

                    // check after current char to give a token for equal and negation
                    //operators
                    // TODO
                    Some('=') => {
                        /* TODO
                        - add start field (as chars ? -> line.chars().next() = self.current)
                        - add current field (as chars ?)
                        - add if is_reach_end method (maybe)
                        */
                        if next_char('=', '=') {
                            TokenType::EqualEqual;
                        } else {
                            TokenType::Equal;
                        }
                    }
                    Some('!') => TokenType::Bang,
                    Some('>') => TokenType::Greater,
                    Some('<') => TokenType::Less,

                    Some('/') => todo!(),
                    Some('\"') => {
                        /*TODO
                            - ingore lexemes from current ` " ` entil the the next ` " `
                            - if didn't find the next ` " ` then return an error
                            - bring back whitespace inside that string
                        */
                        todo!()
                    }

                    _ => {
                        self.tokens.push(Err(ScanErr::UnexpectedLexeme));
                        return self.tokens.clone();
                    }
                };

                let token = Token {
                    token_type,
                    lexeme: "".to_string(),
                    literal: "".to_string(),
                    line: line_num,
                };
                self.tokens.push(Ok(token));

                line.chars().next();
            }

            self.line += line_num;
        }

        self.tokens.push(Ok(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        }));

        return self.tokens.clone();
    }

    fn next_char(current: char, next: char) -> bool {
        if current != next {
            false
        }
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
