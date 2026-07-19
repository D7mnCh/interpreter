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
    fn scan_tokens(&self) -> Result<Token, ScanErr> {}

    // TODO self.tokenize() should push result of self.scan_tokens()
    pub fn tokenize(&mut self) -> Vec<Result<Token, ScanErr>> {
        let source = self.source.clone();
        let source_lines = source.lines();

        for (line_num, line) in source_lines.enumerate() {
            let first_indx = 1;
            // TODO don't whitespace "body" if found
            if line.contains('\"') {
                //handle string
            }
            // NOTE user can have long whitespaced, i don't know later how much that whitespace to converte "body"
            //back with it again
            let lexemes: Vec<&str> = line.split_whitespace().collect();
            for lexeme in lexemes.iter() {
                let token_type = match *lexeme {
                    "(" => TokenType::LeftParen,
                    ")" => TokenType::RightParen,
                    "{" => TokenType::LeftBracet,
                    "}" => TokenType::RightBracet,
                    "," => TokenType::Comma,
                    "." => TokenType::Dot,
                    "-" => TokenType::Minus,
                    "+" => TokenType::Plus,
                    "/" => TokenType::Slash,
                    ";" => TokenType::Semicolon,
                    "*" => TokenType::Asterisk,
                    "=" => TokenType::Equal,
                    "!" => TokenType::Bang,
                    "!=" => TokenType::BangEqual,
                    "==" => TokenType::EqualEqual,
                    "//" => break,
                    "\"" => {
                        /*TODO
                            - ingore lexemes from current ` " ` entil the the next ` " `
                            - if didn't find the next ` " ` then return an error
                            - bring back whitespace inside that string
                        */
                        todo!()
                    }

                    var if false => todo!("if can parse into an number then, it's a num type"),
                    _ => {
                        self.tokens.push(Err(ScanErr::UnexpectedLexeme));
                        return self.tokens.clone();
                    }
                };

                let token = Token {
                    token_type,
                    lexeme: lexeme.to_string(),
                    literal: "".to_string(),
                    line: line_num + first_indx,
                };
                self.tokens.push(Ok(token));
            }

            self.line = line_num + first_indx;
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
