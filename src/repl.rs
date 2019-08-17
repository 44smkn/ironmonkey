use std::io::{self, Write, stdout};

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        stdout().flush().unwrap();

        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_err() {
            return;
        }

        let mut lexer = super::lexer::Lexer::new(&buffer);
        let mut tok = lexer.next_token();
        while tok.token_type.value() != super::token::TokenType::Eof.value() {
            println!("{:?}", tok);
            tok = lexer.next_token();
        }
    }
}
