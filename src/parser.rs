use super::ast::{Identifer, Node, Program, Statement};
use super::lexer::Lexer;
use super::token::Token;

#[derive(Debug)]
struct Parser {
    lexer: Lexer,

    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
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

    fn parse_program<T: Node>(&self) -> Option<Program<T>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::lexer::Lexer;
    use super::*;
    #[test]
    fn let_statement() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program: Program<Statement<Identifer>> = parser
            .parse_program()
            .expect("parse_program(&self) returned None");
    }
}
