pub enum TokenType{
    EOF,
    Identifier, Number,
    Keyword,

    RParen, LParen,
    RBrace, LBrace,
    Semicolon
}

pub struct Token{
    token_type: TokenType,
    value: String,

    line: usize, column: usize
}

impl Token{
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Token{
        return Token{token_type: token_type, value: value, line: line, column: column}
    }
}