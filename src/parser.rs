use super::ast::Program;
use super::lexer::Lexer;
use super::token::Token;

#[derive(Debug)]
struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Default::default(),
            peek_token: Default::default(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&self) -> Program {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn let_statement() {
        unimplemented!();
    }
}