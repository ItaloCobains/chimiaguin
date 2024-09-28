#[cfg(test)]
mod lexer_tests {
    use chimiaguin::lexer::Lexer;
    use chimiaguin::token::Token;

    #[test]
    fn test_single_character_tokens() {
        let mut lexer = Lexer::new("+".to_string());
        assert_eq!(lexer.next_token(), Token::Plus);

        let mut lexer = Lexer::new("-".to_string());
        assert_eq!(lexer.next_token(), Token::Minus);
    }

    #[test]
    fn test_identifier_token() {
        let mut lexer = Lexer::new("a".to_string());
        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
    }

    #[test]
    fn test_number_token() {
        let mut lexer = Lexer::new("5".to_string());
        assert_eq!(lexer.next_token(), Token::Number(5));
    }

    #[test]
    fn test_multiple_tokens() {
        let mut lexer = Lexer::new("a+5-".to_string());

        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Number(5));
        assert_eq!(lexer.next_token(), Token::Minus);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("".to_string());
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
