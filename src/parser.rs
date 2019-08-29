use super::ast::{Identifer, LetStatement, Node, Program, StatementType, ExpressionType};
use super::lexer::Lexer;
use super::token::{Token, TokenType};
use std::mem;

#[derive(Debug)]
struct Parser {
    lexer: Lexer,

    cur_token: Option<Token>,
    peek_token: Option<Token>,
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
        self.cur_token = mem::replace(&mut self.peek_token, None);
        self.peek_token = Some(self.lexer.next_token());
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn peek_token_is(&mut self, token: TokenType) -> bool {
        mem::replace(&mut self.peek_token, None).map_or(false, |v| v.token_type == token)
    }

    fn cur_token_is(&mut self, token:TokenType) -> bool {
        mem::replace(&mut self.cur_token, None).map_or(false, |v| v.token_type == token)
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();
        while mem::replace(&mut self.cur_token, None).map_or(true, |v| v.token_type != TokenType::Eof) {
            program.push(self.parse_statement());
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> StatementType {
        match mem::replace(&mut self.cur_token, None).map_or(TokenType::Illegal, |v| v.token_type) {
            TokenType::Let => self.parse_let_statement(),
            _ => StatementType::Illegal,
        }
    }

    fn parse_let_statement(&mut self) -> StatementType {
        if !self.expect_peek(TokenType::Ident) || !self.expect_peek(TokenType::Assign) {
            return StatementType::Illegal;
        }


        let ident = Identifer {
                token: mem::replace(&mut self.cur_token, None).unwrap(),
                value: mem::replace(&mut self.cur_token, None).map_or(String::new(), |v| v.literal),
        };
        let statement = LetStatement{
            token: mem::replace(&mut self.cur_token, None).unwrap(),
            name: ident.clone(),
            value: ExpressionType::Identifer(ident),
        };

        while self.cur_token_is(TokenType::Semicolon){
            self.next_token();
        }

        StatementType::LetStatement(statement)

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
        let mut parser = Parser::new(lexer);

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
                _ => panic!("fail"),
            };
            assert_eq!(statement.name.value, expected_identifier);
            assert_eq!(statement.name.token_literal(), expected_identifier);
        }
    }
}
