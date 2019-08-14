mod token;
use std::str::Chars;

#[derive(Default, Debug)]
struct Lexer {
    input: &str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: &str) -> &self {
        let lexer = &Lexer{
            input: input,
        }
        lexer
    }

    fn read_char(&mut self) {
        let ch = self.input.get(self.read_position).unwrap_or("\\u{0000}")
    }
}

#[cfg(test)]
mod tests {
    use::super::*;

    struct ExpectedToken {
        type: token::TokenType,
        literal: String,
    }

    impl ExpectedToken {
        fn new_token(type: token::TokenType, literal: &str) -> Self {
            Self {
                type,
                literal: String::from(literal),
            }
        }
    }

    #[test]
    fn next_token() {
        use ExpectedToken::*;
        use token::TokenType::*;

        let input = "=+(){},;";
        let tests = [
            new_token(ASSIGN, "="),
            new_token(PLUS, "+"),
            new_token(LPAREN, "("),
            new_token(RPAREN, ")"),
            new_token(RPAREN, ")"),
            new_token(LBRACE, "{"),
            new_token(RBRACE, "}"),
            new_token(COMMA, ","),
            new_token(SEMICOLON, ";"),
            new_token(EOF, ""),
        ];

        let l = Lexer::new(input);
        for tt in tests.iter() {
            let tok = l.next_token()

            assert_eq!(tok.type, tt.type);
            assert_eq!(tok.literal, tt.literal)
        }
    }
}