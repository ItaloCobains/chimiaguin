#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Plus,
    Minus,
    Eof,
}