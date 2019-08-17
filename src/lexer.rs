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
                    Token::new_token_from_str(Equal, "==")
                } else {
                    Token::new_token_from_char(Assign, self.ch)
                }
            }
            Some(';') => Token::new_token_from_char(Semicolon, self.ch),
            Some('(') => Token::new_token_from_char(Lparen, self.ch),
            Some(')') => Token::new_token_from_char(Rparen, self.ch),
            Some(',') => Token::new_token_from_char(Comma, self.ch),
            Some('+') => Token::new_token_from_char(Plus, self.ch),
            Some('-') => Token::new_token_from_char(Minus, self.ch),
            Some('!') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::new_token_from_str(NotEqual, "!=")
                } else {
                    Token::new_token_from_char(Bang, self.ch)
                }
            }
            Some('*') => Token::new_token_from_char(Asterisk, self.ch),
            Some('/') => Token::new_token_from_char(Slash, self.ch),
            Some('<') => Token::new_token_from_char(Lt, self.ch),
            Some('>') => Token::new_token_from_char(Gt, self.ch),
            Some('{') => Token::new_token_from_char(Lbrace, self.ch),
            Some('}') => Token::new_token_from_char(Rbrace, self.ch),
            None => Token::new_token_from_char(Eof, self.ch),
            _ => {
                if is_letter(self.ch) {
                    ret = true;
                    let literal = self.read_identifer();
                    Token::new_token_from_str(TokenType::lookup_iden(&literal), &literal)
                } else if is_digit(self.ch) {
                    ret = true;
                    Token::new_token_from_str(Int, &self.read_number())
                } else {
                    Token::new_token_from_char(Illegal, self.ch)
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
            ExpectedToken::new_token(Let, "let"),
            ExpectedToken::new_token(Ident, "five"),
            ExpectedToken::new_token(Assign, "="),
            ExpectedToken::new_token(Int, "5"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Let, "let"),
            ExpectedToken::new_token(Ident, "ten"),
            ExpectedToken::new_token(Assign, "="),
            ExpectedToken::new_token(Int, "10"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Let, "let"),
            ExpectedToken::new_token(Ident, "add"),
            ExpectedToken::new_token(Assign, "="),
            ExpectedToken::new_token(Function, "fn"),
            ExpectedToken::new_token(Lparen, "("),
            ExpectedToken::new_token(Ident, "x"),
            ExpectedToken::new_token(Comma, ","),
            ExpectedToken::new_token(Ident, "y"),
            ExpectedToken::new_token(Rparen, ")"),
            ExpectedToken::new_token(Lbrace, "{"),
            ExpectedToken::new_token(Ident, "x"),
            ExpectedToken::new_token(Plus, "+"),
            ExpectedToken::new_token(Ident, "y"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Rbrace, "}"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Let, "let"),
            ExpectedToken::new_token(Ident, "result"),
            ExpectedToken::new_token(Assign, "="),
            ExpectedToken::new_token(Ident, "add"),
            ExpectedToken::new_token(Lparen, "("),
            ExpectedToken::new_token(Ident, "five"),
            ExpectedToken::new_token(Comma, ","),
            ExpectedToken::new_token(Ident, "ten"),
            ExpectedToken::new_token(Rparen, ")"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Bang, "!"),
            ExpectedToken::new_token(Minus, "-"),
            ExpectedToken::new_token(Slash, "/"),
            ExpectedToken::new_token(Asterisk, "*"),
            ExpectedToken::new_token(Int, "5"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Int, "5"),
            ExpectedToken::new_token(Lt, "<"),
            ExpectedToken::new_token(Int, "10"),
            ExpectedToken::new_token(Gt, ">"),
            ExpectedToken::new_token(Int, "5"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(If, "if"),
            ExpectedToken::new_token(Lparen, "("),
            ExpectedToken::new_token(Int, "5"),
            ExpectedToken::new_token(Lt, "<"),
            ExpectedToken::new_token(Int, "10"),
            ExpectedToken::new_token(Rparen, ")"),
            ExpectedToken::new_token(Lbrace, "{"),
            ExpectedToken::new_token(Return, "return"),
            ExpectedToken::new_token(True, "true"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Rbrace, "}"),
            ExpectedToken::new_token(Else, "else"),
            ExpectedToken::new_token(Lbrace, "{"),
            ExpectedToken::new_token(Return, "return"),
            ExpectedToken::new_token(False, "false"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Rbrace, "}"),
            ExpectedToken::new_token(Int, "10"),
            ExpectedToken::new_token(Equal, "=="),
            ExpectedToken::new_token(Int, "10"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Int, "10"),
            ExpectedToken::new_token(NotEqual, "!="),
            ExpectedToken::new_token(Int, "9"),
            ExpectedToken::new_token(Semicolon, ";"),
            ExpectedToken::new_token(Eof, ""),
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
