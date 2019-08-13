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
