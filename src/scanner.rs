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

    // NOTE scanning shouldn't take mutable reference, scanning =! writing
    pub fn scan_tokens(&mut self) -> Vec<Result<Token, ScanErr>> {
        let source = self.source.clone();
        let source_lines = source.lines();

        for (line_num, line) in source_lines.enumerate() {
            let lexemes: Vec<&str> = line.split_whitespace().collect();
            let some_token = self.scan_token(&lexemes, line_num + 1);

            if let Some(_scan_err) = some_token {
                return self.tokens.clone();
            }

            self.line = line_num;
        }

        self.tokens.push(Ok(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        }));

        return self.tokens.clone();
    }

    fn scan_token(&mut self, lexemes: &[&str], line_num: usize) -> Option<ScanErr> {
        for lexeme in lexemes {
            // NOTE maybe add a tokinazier method that i think the maining of it is will convert a lexeme into a token, or rename method with that name if true?
            let token_type = match *lexeme {
                "(" => TokenType::LEFT_PAREN,
                ")" => TokenType::RIGHT_PAREN,
                "{" => TokenType::LEFT_BRACE,
                "}" => TokenType::RIGHT_BRACE,
                "," => TokenType::COMMA,
                "." => TokenType::DOT,
                "+" => TokenType::PLUS,
                "-" => TokenType::MINUS,
                ";" => TokenType::SEMICOLON,
                "*" => TokenType::ASTERISK,
                _ => {
                    self.tokens.push(Err(ScanErr::UnexpectedLexeme));
                    return Some(ScanErr::UnexpectedLexeme);
                }
            };

            let token = Token {
                token_type,
                lexeme: lexeme.to_string(),
                literal: "".to_string(),
                line: line_num,
            };
            self.tokens.push(Ok(token));
        }

        None
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
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    ASTERISK,

    // One or two character tokens.
    BANG, // !
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    STRUCT,
    ELSE,
    FALSE,
    FUNC,
    FOR,
    IF,
    NULL,
    OR,
    PRINT,
    RETURN,
    SELF,
    TRUE,
    VAR,
    WHILE,

    EOF,
}
