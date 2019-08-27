use super::ast::{Identifer, LetStatement, Node, Program};
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

    fn parse_program(&self) -> Program {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::super::ast::StatementType;
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

        let program: Program = parser.parse_program();

        assert_eq!(
            program.len(),
            3,
            "program does not contain 3 statements. got={}",
            program.len()
        );

        let tests = vec!["x", "y", "foobar"];
        for (expected_identifier, statement) in tests.into_iter().zip(program.iter()) {
            assert_eq!(
                statement.token_literal(),
                "let",
                "statement.token_literal not 'let'. got={}",
                statement.token_literal()
            );
            let statement = match statement {
                StatementType::LetStatement(statement) => statement,
            };
            assert_eq!(statement.name.value, expected_identifier);
            assert_eq!(statement.name.token_literal(), expected_identifier);
        }
    }
}
