enum ScanErr {}

impl ScanErr {
    pub fn report(&self) {
        // TODO match self
        println!("Error: found syntax error");
    }
}

pub struct Scanner {
    source: String,
    // NOTE i think scan_token should have Result,
    // Scanner should have available tokens
    tokens: &[Token],
    // NOTE those fields for lexemes
    //start: usize,
    //current: usize,
    //line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let token = Self::scan_tokens(&source);

        Self { source, tokens }
    }

    fn scan_tokens() -> &[Token] {}
    fn scan_token() -> Type {}
}

pub struct Type {
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
    STAR,

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
