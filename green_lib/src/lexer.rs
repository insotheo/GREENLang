use crate::token;
use crate::token::{Token, TokenType};

pub struct Lexer{
    input: String,
    index: usize, column: usize, line: usize,
    pub errors: Vec<String>
}

impl Lexer {
    pub fn new(input: &str) -> Lexer{
        return Lexer{input: String::from(input), index: 0, column: 1, line: 1, errors: Vec::new()};
    }

    pub fn tokenize(&mut self) -> Vec<token::Token>{
        let mut tokens: Vec<Token> = Vec::new();

        while self.is_in_bound(){
            let mut c: char = self.get_current_char();

            if c.is_whitespace() {
                self.next();
                continue;
            }

            if c.is_numeric() {
                let mut number: String = String::new();
                let mut has_point: bool = false;
                while self.is_in_bound() && (c.is_numeric() || c == '.'){
                    if c == '.'{
                        if has_point{
                            self.panic("Incorrect number format!");
                        }
                        has_point = true;
                    }
                    number.push(c);
                    self.next();
                    c = self.get_current_char();
                }
                tokens.push(Token::new(TokenType::Number, number, self.line, self.column));
                continue;
            }

            if c.is_alphabetic() || c == '_' {
                let mut identifier: String = String::new();
                while self.is_in_bound() && (c.is_alphanumeric() || c == '_') {
                    identifier.push(c);
                    self.next();
                    c = self.get_current_char();
                }

                match identifier.as_str() {
                    "ret" => tokens.push(Token::new(TokenType::Keyword, identifier, self.line, self.column)),
                    _ => tokens.push(Token::new(TokenType::Identifier, identifier, self.line, self.column))
                }

                continue;
            }

            match c{
                ';' => tokens.push(Token::new(TokenType::Semicolon, String::new(), self.line, self.column)),
                '(' => tokens.push(Token::new(TokenType::LParen, String::new(), self.line, self.column)),
                ')' => tokens.push(Token::new(TokenType::RParen, String::new(), self.line, self.column)),
                '{' => tokens.push(Token::new(TokenType::LBrace, String::new(), self.line, self.column)),
                '}' => tokens.push(Token::new(TokenType::RBrace, String::new(), self.line, self.column)),
                _ => self.panic(format!("Unknown token('{}')", c).as_str())
            }
            self.next();
        }
        tokens.push(Token::new(TokenType::EOF, "".to_string(), 0,  self.line + 1));

        return tokens;
    }

    fn is_in_bound(&self) -> bool{
        return self.index < self.input.chars().count();
    }

    fn next(&mut self){
        self.index += 1;
        self.column += 1;

        if !self.is_in_bound(){
            return;
        }

        if self.get_current_char() == '\n'{
            self.index += 1;
            self.line += 1;
            self.column = 1;
        }
    }

    fn get_current_char(&self) -> char{
        return self.input.chars().nth(self.index).unwrap();
    }

    fn panic(&mut self, err_msg: &str){
        let line: String = self.input.lines().nth(self.line - 1).unwrap().to_string();
        self.errors.push(format!("LEXER ERROR({}:{}): {}: {}", self.line, self.column, err_msg, line));
    }

    pub fn is_success(&self) -> bool{
        return self.errors.iter().count() == 0;
    }
}