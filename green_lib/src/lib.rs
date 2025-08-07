pub mod token;
pub mod lexer;

pub fn get_version() -> String{
    return std::env!("CARGO_PKG_VERSION").to_string();
}