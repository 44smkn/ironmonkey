use super::ast::{ExpressionType, Identifer, LetStatement, Node, Program, StatementType};
use super::lexer::Lexer;
use super::token::{Token, TokenType};
use std::mem;

#[derive(Debug, Clone)]
struct Parser {
    lexer: Lexer,

    cur_token: Option<Box<Token>>,
    peek_token: Option<Box<Token>>,
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
        self.peek_token = Some(Box::from(self.lexer.next_token()));
        println!(
            "cur_token={}, peek_token={}",
            self.cur_token
                .clone()
                .map_or(String::from("None"), |v| v.literal),
            self.peek_token
                .clone()
                .map_or(String::from("None"), |v| v.literal)
        );
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
        self.peek_token
            .clone()
            .map_or(false, |v| v.token_type == token)
    }

    fn cur_token_is(&mut self, token: TokenType) -> bool {
        self.cur_token
            .clone()
            .map_or(false, |v| v.token_type == token)
    }

    fn parse_program(&mut self) -> Program {
        let mut program: Vec<StatementType> = Vec::new();
        while self
            .cur_token
            .clone()
            .map_or(true, |v| v.token_type != TokenType::Eof)
        {
            let statement = self.parse_statement();
            match statement {
                StatementType::Illegal => println!(
                    "stetement type is illegal.token={}",
                    self.cur_token
                        .clone()
                        .map_or(String::from("None"), |v| v.literal)
                ),
                _ => program.push(statement),
            };
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> StatementType {
        match self
            .cur_token
            .clone()
            .map_or(TokenType::Illegal, |v| v.token_type)
        {
            TokenType::Let => self.parse_let_statement(),
            _ => StatementType::Illegal,
        }
    }

    fn parse_let_statement(&mut self) -> StatementType {
        let first_token = self.cur_token.clone().unwrap_or(Box::new(Token {
            ..Default::default()
        }));

        if !self.expect_peek(TokenType::Ident) {
            return StatementType::Illegal;
        }

        let second_token = self.cur_token.clone().unwrap_or(Box::new(Token {
            ..Default::default()
        }));
        let ident = Identifer {
            token: second_token.clone(),
            value: second_token.clone().literal,
        };
        let statement = LetStatement {
            token: first_token,
            name: ident.clone(),
            value: ExpressionType::Identifer(ident),
        };

        if !self.expect_peek(TokenType::Assign) {
            return StatementType::Illegal;
        }

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        println!("return let statement: {}", statement.token.literal);
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
        println!("{}", lexer);
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
