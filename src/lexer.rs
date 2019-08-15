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

        let tok = match self.ch {
            Some('=') => Token::new_token(ASSIGN, self.ch.unwrap()),
            Some(';') => Token::new_token(SEMICOLON, self.ch.unwrap()),
            Some('(') => Token::new_token(LPAREN, self.ch.unwrap()),
            Some(')') => Token::new_token(RPAREN, self.ch.unwrap()),
            Some(',') => Token::new_token(COMMA, self.ch.unwrap()),
            Some('+') => Token::new_token(PLUS, self.ch.unwrap()),
            Some('{') => Token::new_token(LBRACE, self.ch.unwrap()),
            Some('}') => Token::new_token(RBRACE, self.ch.unwrap()),
            None => Token::new_token(EOF, 0x00_u8.into()),
            _ => Token::new_token(ILLEGAL, self.ch.unwrap()),
        };

        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct ExpectedToken {
        expected_type: token::TokenType,
        literal: char,
    }

    impl ExpectedToken {
        fn new_token(expected_type: token::TokenType, literal: char) -> Self {
            Self {
                expected_type,
                literal,
            }
        }
    }

    #[test]
    fn next_token_confirm() {
        use super::token::TokenType::*;
        use super::*;

        let nul: char = 0x00_u8.into();
        let input = "=+(){},;";
        let tests = [
            ExpectedToken::new_token(ASSIGN, '='),
            ExpectedToken::new_token(PLUS, '+'),
            ExpectedToken::new_token(LPAREN, '('),
            ExpectedToken::new_token(RPAREN, ')'),
            ExpectedToken::new_token(LBRACE, '{'),
            ExpectedToken::new_token(RBRACE, '}'),
            ExpectedToken::new_token(COMMA, ','),
            ExpectedToken::new_token(SEMICOLON, ';'),
            ExpectedToken::new_token(EOF, nul),
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
