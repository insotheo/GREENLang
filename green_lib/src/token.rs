pub enum TokenType{
    EOF,
    Identifier, Number,
    Keyword,

    RParen, LParen,
    RBrace, LBrace
}

pub struct Token{
    token_type: TokenType,
    value: String,

    line: u32, column: u32
}