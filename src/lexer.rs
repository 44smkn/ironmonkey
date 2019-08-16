use super::token;

#[derive(Default, Debug)]
struct Lexer {
    input: Vec<char>,
    position: usize,      // 現在検査中のchの位置を指し示す
    read_position: usize, // 入力における「次の」位置を指し示す
    ch: Option<char>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            ..Default::default()
        };
        lexer.read_char(); // position, read_position, chの初期化
        lexer
    }

    fn read_char(&mut self) {
        self.ch = self.input.get(self.read_position).map(|ch| *ch);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> token::Token {
        use token::TokenType::*;
        use token::*;

        let to_literal: fn(&Lexer) -> String = |l| l.ch.unwrap().to_string();
        let tok = match self.ch {
            Some('=') => Token::new_token(ASSIGN, to_literal(self)),
            Some(';') => Token::new_token(SEMICOLON, to_literal(self)),
            Some('(') => Token::new_token(LPAREN, to_literal(self)),
            Some(')') => Token::new_token(RPAREN, to_literal(self)),
            Some(',') => Token::new_token(COMMA, to_literal(self)),
            Some('+') => Token::new_token(PLUS, to_literal(self)),
            Some('{') => Token::new_token(LBRACE, to_literal(self)),
            Some('}') => Token::new_token(RBRACE, to_literal(self)),
            None => Token::new_token(EOF, "".to_string()),
            _ => Token::new_token(ILLEGAL, to_literal(self)),
        };

        self.read_char();
        tok
    }

    fn skip_white_space(&self) -> bool {
        self.ch.as_ref().map_or(false, char::is_ascii_whitespace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct ExpectedToken {
        expected_type: token::TokenType,
        literal: String,
    }

    impl ExpectedToken {
        fn new_token(expected_type: token::TokenType, literal: &str) -> Self {
            Self {
                expected_type,
                literal: literal.to_string(),
            }
        }
    }

    #[test]
    fn next_token_confirm() {
        use super::token::TokenType::*;
        use super::*;

        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
";
        let tests = [
            ExpectedToken::new_token(LET, "let"),
            ExpectedToken::new_token(IDENT, "five"),
            ExpectedToken::new_token(ASSIGN, "="),
            ExpectedToken::new_token(INT, "5"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(LET, "let"),
            ExpectedToken::new_token(IDENT, "ten"),
            ExpectedToken::new_token(ASSIGN, "="),
            ExpectedToken::new_token(INT, "10"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(LET, "let"),
            ExpectedToken::new_token(IDENT, "add"),
            ExpectedToken::new_token(ASSIGN, "="),
            ExpectedToken::new_token(FUNCTION, "fn"),
            ExpectedToken::new_token(LPAREN, "("),
            ExpectedToken::new_token(IDENT, "x"),
            ExpectedToken::new_token(COMMA, ","),
            ExpectedToken::new_token(IDENT, "y"),
            ExpectedToken::new_token(RPAREN, ")"),
            ExpectedToken::new_token(LBRACE, "{"),
            ExpectedToken::new_token(IDENT, "x"),
            ExpectedToken::new_token(PLUS, "+"),
            ExpectedToken::new_token(IDENT, "y"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(RBRACE, "}"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(LET, "let"),
            ExpectedToken::new_token(IDENT, "result"),
            ExpectedToken::new_token(ASSIGN, "="),
            ExpectedToken::new_token(IDENT, "add"),
            ExpectedToken::new_token(LPAREN, "("),
            ExpectedToken::new_token(IDENT, "five"),
            ExpectedToken::new_token(COMMA, ","),
            ExpectedToken::new_token(IDENT, "ten"),
            ExpectedToken::new_token(RPAREN, ")"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(EOF, ""),
        ];

        let mut l = Lexer::new(input);
        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();

            assert!(
                tok.token_type.value() == tt.expected_type.value(),
                "tests[{}], - tokentype wrong. expected={}, got={}",
                i,
                tt.expected_type.value(),
                tok.token_type.value()
            );
            assert_eq!(
                tok.literal,
                tt.literal,
                "tests[{}], - literal wrong. expected={}, got={}",
                i,
                tt.expected_type.value(),
                tok.token_type.value()
            );
        }
    }
}
