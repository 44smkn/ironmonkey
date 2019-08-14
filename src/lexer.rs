use super::token;

#[derive(Default, Debug)]
struct Lexer {
    input: Vec<char>,
    position: usize,      // 現在検査中のchの位置を指し示す
    read_position: usize, // 入力における「次の」位置を指し示す
    ch: char,
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
        if self.read_position >= self.input.len() {
            // 終端に到達したらNUL文字を入れる
            self.ch = 0x00_u8.into();
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> token::Token {
        use token::TokenType::*;
        use token::*;

        let nul: char = 0x00_u8.into();
        let tok = match self.ch {
            '=' => Token::new_token(ASSIGN, self.ch),
            ';' => Token::new_token(SEMICOLON, self.ch),
            '(' => Token::new_token(LPAREN, self.ch),
            ')' => Token::new_token(RPAREN, self.ch),
            ',' => Token::new_token(COMMA, self.ch),
            '+' => Token::new_token(PLUS, self.ch),
            '{' => Token::new_token(LBRACE, self.ch),
            '}' => Token::new_token(RBRACE, self.ch),
            nul => Token::new_token(EOF, nul),
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
        token_type: token::TokenType,
        literal: char,
    }

    impl ExpectedToken {
        fn new_token(token_type: token::TokenType, literal: char) -> Self {
            Self {
                token_type,
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
        for tt in tests.iter() {
            let tok = l.next_token();

            assert!(tok.token_type.value() == tt.token_type.value());
            assert_eq!(tok.literal, tt.literal);
        }
    }
}
