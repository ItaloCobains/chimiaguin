use crate::token::Token;

pub struct Lexer {
  input: String,
  position: usize,
}

impl Lexer {
  pub fn new(input: String) -> Lexer {
    Lexer {
      input,
      position: 0,
    }
  }

  pub fn next_token(&mut self) -> Token {
    if self.position >= self.input.len() {
      return Token::Eof;
    }

    let ch = self.input.chars().nth(self.position).unwrap();
    self.position += 1;

    match ch {
      '+' => Token::Plus,
      '-' => Token::Minus,
      c if c.is_alphabetic() => Token::Identifier(c.to_string()),
      c if c.is_numeric() => Token::Number(c.to_digit(10).unwrap() as i32),
      _ => Token::Eof,
    }
  }
}