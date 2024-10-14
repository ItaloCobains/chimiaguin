#[cfg(test)]
mod lexer_tests {
    use chimiaguin::lexer::Lexer;
    use chimiaguin::token::Token;

    #[test]
    fn test_single_character_tokens() {
        let mut lexer = Lexer::new("+");
        assert_eq!(lexer.next_token(), Token::Plus);

        let mut lexer = Lexer::new("-");
        assert_eq!(lexer.next_token(), Token::Minus);
    }

    #[test]
    fn test_identifier_token() {
        let mut lexer = Lexer::new("a");
        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
    }

    #[test]
    fn test_number_token() {
        let mut lexer = Lexer::new("5");
        assert_eq!(lexer.next_token(), Token::Number(5));
    }

    #[test]
    fn test_multiple_tokens() {
        let mut lexer = Lexer::new("a+5-");

        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Number(5));
        assert_eq!(lexer.next_token(), Token::Minus);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_whitespace() {
        let mut lexer = Lexer::new("test \n");

        assert_eq!(lexer.next_token(), Token::Identifier("test".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::BreakLine);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_bigger_number() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.next_token(), Token::Number(123));
    }

    #[test]
    fn test_identifier_with_multiple_characters() {
        let mut lexer = Lexer::new("abc");
        assert_eq!(lexer.next_token(), Token::Identifier("abc".to_string()));
    }

    #[test]
    fn test_hello_world_ruby() {
        let mut lexer = Lexer::new("puts 'Hello, World!'");

        assert_eq!(lexer.next_token(), Token::Identifier("puts".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Text("Hello, World!".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_function_call() {
        let mut lexer = Lexer::new("add(1, 2)");

        assert_eq!(lexer.next_token(), Token::Identifier("add".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftParenthesis);
        assert_eq!(lexer.next_token(), Token::Number(1));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Number(2));
        assert_eq!(lexer.next_token(), Token::RightParenthesis);
    }

    #[test]
    fn test_function_call_with_text() {
        let mut lexer = Lexer::new("add('hello', 'world')");

        assert_eq!(lexer.next_token(), Token::Identifier("add".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftParenthesis);
        assert_eq!(lexer.next_token(), Token::Text("hello".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Text("world".to_string()));
        assert_eq!(lexer.next_token(), Token::RightParenthesis);
    }

    #[test]
    fn test_function_call_with_text_and_number() {
        let mut lexer = Lexer::new("add('hello', 5)");

        assert_eq!(lexer.next_token(), Token::Identifier("add".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftParenthesis);
        assert_eq!(lexer.next_token(), Token::Text("hello".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Number(5));
        assert_eq!(lexer.next_token(), Token::RightParenthesis);
    }

    #[test]
    fn test_function_call_with_text_and_number_and_text() {
        let mut lexer = Lexer::new("add('hello', 5, 'world')");

        assert_eq!(lexer.next_token(), Token::Identifier("add".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftParenthesis);
        assert_eq!(lexer.next_token(), Token::Text("hello".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Number(5));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Text("world".to_string()));
        assert_eq!(lexer.next_token(), Token::RightParenthesis);
    }

    #[test]
    fn test_declare_function_without_params() {
        let mut lexer = Lexer::new(
            "def add\n\
            end",
        );
        
        assert_eq!(lexer.next_token(), Token::Identifier("def".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("add".to_string()));
        assert_eq!(lexer.next_token(), Token::BreakLine);
        assert_eq!(lexer.next_token(), Token::Identifier("end".to_string()))
    }

    #[test]
    fn test_declare_function_with_params() {
        let mut lexer = Lexer::new(
            "def add(a, b)\n\
            end",
        );

        assert_eq!(lexer.next_token(), Token::Identifier("def".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("add".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftParenthesis);
        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));
        assert_eq!(lexer.next_token(), Token::RightParenthesis);
        assert_eq!(lexer.next_token(), Token::BreakLine);
        assert_eq!(lexer.next_token(), Token::Identifier("end".to_string()))
    }

    #[test]
    fn test_declare_class() {
        let mut lexer = Lexer::new(
            "class Dog\nend"
        );

        assert_eq!(lexer.next_token(), Token::Identifier("class".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("Dog".to_string()));
        assert_eq!(lexer.next_token(), Token::BreakLine);
        assert_eq!(lexer.next_token(), Token::Identifier("end".to_string()));
    }

    #[test]
    fn test_declare_class_with_father() {
        let mut lexer = Lexer::new(
            "class Dog < Animal\nend"
        );

        assert_eq!(lexer.next_token(), Token::Identifier("class".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("Dog".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::LessThan);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("Animal".to_string()));
        assert_eq!(lexer.next_token(), Token::BreakLine);
        assert_eq!(lexer.next_token(), Token::Identifier("end".to_string()));
    }

    #[test]
    fn test_equals() {
        let mut lexer = Lexer::new("==");

        assert_eq!(lexer.next_token(), Token::EqualEqual);

        let mut lexer = Lexer::new("===");
        assert_eq!(lexer.next_token(), Token::EqualEqualEqual);

        let mut lexer = Lexer::new("=>");
        assert_eq!(lexer.next_token(), Token::Arrow);

        let mut lexer = Lexer::new("=");
        assert_eq!(lexer.next_token(), Token::Equal);
    }

    #[test]
    fn test_comparison_operators() {
        let mut lexer = Lexer::new("<= >= < >");
        assert_eq!(lexer.next_token(), Token::LessThanOrEqual);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::GreaterThanOrEqual);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::LessThan);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::GreaterThan);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_arithmetic_operators() {
        let mut lexer = Lexer::new("+ - * / %");
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Minus);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Asterisk);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Percent);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_string_with_interpolation() {
        let mut lexer = Lexer::new("\"Hello, #{name}!\"");
        assert_eq!(lexer.next_token(), Token::Interpolation("Hello, ".to_string(), "name".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_symbol_token() {
        let mut lexer = Lexer::new(":symbol");
        assert_eq!(lexer.next_token(), Token::Symbol("symbol".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_block_delimiters() {
        let mut lexer = Lexer::new("{ }");
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_hash_declaration() {
        let mut lexer = Lexer::new("{ :key => 'value' }");

        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Symbol("key".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Arrow);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Text("value".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_function_declaration_with_block() {
        let mut lexer = Lexer::new("def foo\n  yield\nend");

        assert_eq!(lexer.next_token(), Token::Identifier("def".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("foo".to_string()));
        assert_eq!(lexer.next_token(), Token::BreakLine);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("yield".to_string()));
        assert_eq!(lexer.next_token(), Token::BreakLine);
        assert_eq!(lexer.next_token(), Token::Identifier("end".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("a = { b: 1, c: 'hello' }");

        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Equal);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Number(1));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier("c".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Text("hello".to_string()));
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
