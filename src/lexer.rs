use super::token;

#[derive(Default, Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,      // 現在検査中のchの位置を指し示す
    read_position: usize, // 入力における「次の」位置を指し示す
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
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

    pub fn next_token(&mut self) -> token::Token {
        use token::TokenType::*;
        use token::*;

        self.skip_white_space();

        // TODO: early returnのための対応をスマートにする
        let mut ret = false;
        let tok = match self.ch {
            Some('=') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::new_token_from_str(EQ, "==")
                } else {
                    Token::new_token_from_char(ASSIGN, self.ch)
                }
            }
            Some(';') => Token::new_token_from_char(SEMICOLON, self.ch),
            Some('(') => Token::new_token_from_char(LPAREN, self.ch),
            Some(')') => Token::new_token_from_char(RPAREN, self.ch),
            Some(',') => Token::new_token_from_char(COMMA, self.ch),
            Some('+') => Token::new_token_from_char(PLUS, self.ch),
            Some('-') => Token::new_token_from_char(MINUS, self.ch),
            Some('!') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::new_token_from_str(NOT_EQ, "!=")
                } else {
                    Token::new_token_from_char(BANG, self.ch)
                }
            },
            Some('*') => Token::new_token_from_char(ASTERRISK, self.ch),
            Some('/') => Token::new_token_from_char(SLASH, self.ch),
            Some('<') => Token::new_token_from_char(LT, self.ch),
            Some('>') => Token::new_token_from_char(GT, self.ch),
            Some('{') => Token::new_token_from_char(LBRACE, self.ch),
            Some('}') => Token::new_token_from_char(RBRACE, self.ch),
            None => Token::new_token_from_char(EOF, self.ch),
            _ => {
                if is_letter(self.ch) {
                    ret = true;
                    let literal = self.read_identifer();
                    Token::new_token_from_str(TokenType::lookup_iden(&literal), &literal)
                } else if is_digit(self.ch) {
                    ret = true;
                    Token::new_token_from_str(INT, &self.read_number())
                } else {
                    Token::new_token_from_char(ILLEGAL, self.ch)
                }
            }
        };

        if ret {
            return tok;
        };
        self.read_char();
        tok
    }

    fn skip_white_space(&mut self) {
        while self.ch.as_ref().map_or(false, char::is_ascii_whitespace) {
            self.read_char();
        }
    }

    fn read_identifer(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].into_iter().collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].into_iter().collect()
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.read_position).map(|v| *v)
    }
}

fn is_letter(ch: Option<char>) -> bool {
    ch.map_or(false, |v| v.is_ascii_alphabetic() || v == '_')
}

fn is_digit(ch: Option<char>) -> bool {
    ch.as_ref().map_or(false, char::is_ascii_digit)
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

        let input = "
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
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
            ExpectedToken::new_token(BANG, "!"),
            ExpectedToken::new_token(MINUS, "-"),
            ExpectedToken::new_token(SLASH, "/"),
            ExpectedToken::new_token(ASTERRISK, "*"),
            ExpectedToken::new_token(INT, "5"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(INT, "5"),
            ExpectedToken::new_token(LT, "<"),
            ExpectedToken::new_token(INT, "10"),
            ExpectedToken::new_token(GT, ">"),
            ExpectedToken::new_token(INT, "5"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(IF, "if"),
            ExpectedToken::new_token(LPAREN, "("),
            ExpectedToken::new_token(INT, "5"),
            ExpectedToken::new_token(LT, "<"),
            ExpectedToken::new_token(INT, "10"),
            ExpectedToken::new_token(RPAREN, ")"),
            ExpectedToken::new_token(LBRACE, "{"),
            ExpectedToken::new_token(RETURN, "return"),
            ExpectedToken::new_token(TRUE, "true"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(RBRACE, "}"),
            ExpectedToken::new_token(ELSE, "else"),
            ExpectedToken::new_token(LBRACE, "{"),
            ExpectedToken::new_token(RETURN, "return"),
            ExpectedToken::new_token(FALSE, "false"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(RBRACE, "}"),
            ExpectedToken::new_token(INT, "10"),
            ExpectedToken::new_token(EQ, "=="),
            ExpectedToken::new_token(INT, "10"),
            ExpectedToken::new_token(SEMICOLON, ";"),
            ExpectedToken::new_token(INT, "10"),
            ExpectedToken::new_token(NOT_EQ, "!="),
            ExpectedToken::new_token(INT, "9"),
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
