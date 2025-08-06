use crate::token;

pub struct Lexer{
    input: String,
    index: usize
}

impl Lexer {
    pub fn new(input: &str) -> Lexer{
        return Lexer{input: String::from(input), index: 0};
    }

    pub fn tokenize(&mut self) -> Vec<token::Token>{
        let mut tokens: Vec<token::Token> = Vec::new();

        while self.is_in_bound(){
            
        }

        return tokens;
    }

    fn is_in_bound(&self) -> bool{
        return self.index < self.input.chars().count();
    }
}