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
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
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
        // NOTE very expansive clone, i don't wanna clone
        let total_num_lines = source_lines.clone().count();
        dbg!(total_num_lines);

        // NOTE if range is x..x, it will not execute
        for (line, line_num) in source_lines.zip(1..total_num_lines) {
            //println!("found a line to iterate over chars");
            for charac in line.chars() {
                let token_type = match charac {
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '{' => TokenType::LeftBracet,
                    '}' => TokenType::RightBracet,
                    ',' => TokenType::Comma,
                    '.' => TokenType::Dot,
                    '-' => TokenType::Minus,
                    '+' => TokenType::Plus,
                    '/' => TokenType::Slash,
                    ';' => TokenType::Semicolon,
                    '*' => TokenType::Asterisk,

                    // check after current char to give a token for  equal and negation
                    //operators
                    // TODO
                    '=' => TokenType::Equal,
                    '!' => TokenType::Bang,

                    '/' => todo!(),
                    '\"' => {
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
            }

            self.line = line_num;
        }

        self.tokens.push(Ok(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        }));

        return self.tokens.clone();
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
