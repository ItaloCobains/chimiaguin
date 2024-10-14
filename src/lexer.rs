use crate::token::Token;

pub struct Lexer<'a> {
    position: usize,
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        Lexer {
            position: 0,
            chars,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next();
        self.position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(ch) = self.current_char {
            match ch {
                ':' => {
                    self.advance();
                    self.resolve_colon_or_symbol()
                },
                '{' => {
                    self.advance();
                    Token::LeftBrace
                },
                '}' => {
                    self.advance();
                    Token::RightBrace
                },
                '*' => {
                    self.advance();
                    Token::Asterisk
                },
                '/' => {
                    self.advance();
                    Token::Slash
                },
                '%' => {
                    self.advance();
                    Token::Percent
                },
                '=' => {
                    self.advance();
                    self.resolve_equal()
                }
                '<' => {
                    self.advance();
                    self.resolve_less()
                }
                '>' => {
                    self.advance();
                    self.resolve_greater()
                }
                '(' => {
                    self.advance();
                    Token::LeftParenthesis
                }
                ')' => {
                    self.advance();
                    Token::RightParenthesis
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                '\'' | '"' => {
                    self.advance();
                    self.read_string(ch)
                }
                ' ' => {
                    self.advance();
                    Token::WhiteSpace
                }
                '\n' => {
                    self.advance();
                    Token::BreakLine
                }
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                c if c.is_alphabetic() => {
                    self.advance();
                    self.read_identifier(c)
                }
                c if c.is_numeric() => {
                    self.advance();
                    self.read_number(c)
                }
                _ => {
                    self.advance();
                    Token::Illegal(ch.to_string())
                }
            }
        } else {
            Token::Eof
        }
    }

    fn resolve_greater(&mut self) -> Token {
        match self.current_char {
            Some('=') => {
                self.advance();
                Token::GreaterThanOrEqual
            }
            _ => Token::GreaterThan,  // Não avança se não for '='
        }
    }

    fn resolve_less(&mut self) -> Token {
        match self.current_char {
            Some('=') => {
                self.advance();
                Token::LessThanOrEqual
            }
            _ => Token::LessThan,  // Não avança se não for '='
        }
    }

    fn resolve_equal(&mut self) -> Token {
        match self.current_char {
            Some('=') => {
                self.advance();
                match self.current_char {
                    Some('=') => {
                        self.advance();
                        Token::EqualEqualEqual
                    }
                    _ => Token::EqualEqual,
                }
            }
            Some('>') => {
                self.advance();
                Token::Arrow
            }
            _ => Token::Equal,
        }
    }

    fn read_string(&mut self, quote: char) -> Token {
        let mut text = String::new();
        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance();
                break;
            }
            text.push(ch);
            self.advance();
        }
        Token::Text(text)
    }

    fn read_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first_char);

        while let Some(ch) = self.current_char {
            if !ch.is_alphabetic() {
                break;
            }
            identifier.push(ch);
            self.advance();
        }

        Token::Identifier(identifier)
    }

    fn read_number(&mut self, first_char: char) -> Token {
        let mut number = String::new();
        number.push(first_char);

        while let Some(ch) = self.current_char {
            if !ch.is_numeric() {
                break;
            }
            number.push(ch);
            self.advance();
        }

        Token::Number(number.parse::<i32>().unwrap_or(0))
    }

    // Should resolve the token for colon or symbol
    // If the next char is a letter, it should be a symbol
    // Otherwise, it should be a colon
    fn resolve_colon_or_symbol(&mut self) -> Token {
        match self.current_char {
            Some(ch) if ch.is_alphabetic() => {
                self.advance();
                self.read_symbol(ch)
            }
            _ => Token::Colon,
        }
    }

    fn read_symbol(&mut self, first_char: char) -> Token {
        let mut symbol = String::new();
        symbol.push(first_char);

        while let Some(ch) = self.current_char {
            if !ch.is_alphabetic() {
                break;
            }
            symbol.push(ch);
            self.advance();
        }

        Token::Symbol(symbol)
    }
}
