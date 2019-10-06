use super::ast::{
    ExpressionStatement, ExpressionType, Identifer, LetStatement, Program, ReturnStatement,
    StatementType,
};
use super::lexer::Lexer;
use super::token::{Token, TokenType};
use crate::ast::Node;
use std::collections::HashMap;
use std::mem;

// type alias
type PrefixParseFn = fn() -> ExpressionType;
type InfixParseFn = fn(ExpressionType) -> ExpressionType;

#[derive(Debug, Clone)]
struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    cur_token: Option<Box<Token>>,
    peek_token: Option<Box<Token>>,

    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            errors: Vec::new(),
            cur_token: Default::default(),
            peek_token: Default::default(),
            prefix_parse_fns: Default::default(),
            infix_parse_fns: Default::default(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, token_type: &TokenType) {
        let message = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type,
            discover_token_type(&self.peek_token)
        );
        self.errors.push(message);
    }

    fn next_token(&mut self) {
        self.cur_token = mem::replace(&mut self.peek_token, None);
        self.peek_token = Some(Box::from(self.lexer.next_token()));
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(&token) {
            true
        } else {
            self.peek_error(&token);
            false
        }
    }

    fn peek_token_is(&mut self, token: &TokenType) -> bool {
        discover_token_type(&self.peek_token) == *token
    }

    fn cur_token_is(&mut self, token: TokenType) -> bool {
        discover_token_type(&self.cur_token) == token
    }

    /// Repeat to read by calling next_token() token until reaching TokenType::Eof.
    /// Every time it repeats, call parse_statement() that analysis statement.
    fn parse_program(&mut self) -> Program {
        let mut program: Vec<StatementType> = Vec::new();
        while discover_token_type(&self.cur_token) != TokenType::Eof {
            let statement = self.parse_statement();
            program.push(statement);
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> StatementType {
        match discover_token_type(&self.cur_token) {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// Construct LetStatement node based on token(LET) what we focus on.
    /// Provide assertion following tokens and advance a token by calling expect_peek().
    /// First, I expect TokenType::Ident. It is used for constructing Identifer node.
    /// Then, I expect equal and jump until semicolon.
    fn parse_let_statement(&mut self) -> StatementType {
        let first_token = match mem::replace(&mut self.cur_token, None) {
            Some(token) => token,
            None => panic!("not found current token"),
        };

        if !self.expect_peek(TokenType::Ident) {
            // TODO: return Result error
        }
        self.next_token();

        let second_token = match mem::replace(&mut self.cur_token, None) {
            Some(token) => token,
            None => panic!("not found current token"),
        };

        let ident = Identifer::new(second_token);
        let statement = LetStatement {
            token: first_token,
            name: ident.clone(),
            value: ExpressionType::Identifer(ident),
        };

        if !self.expect_peek(TokenType::Assign) {
            // TODO: return Result error
        }
        self.next_token();

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        StatementType::LetStatement(statement)
    }

    fn parse_return_statement(&mut self) -> StatementType {
        let token = match mem::replace(&mut self.cur_token, None) {
            Some(token) => token,
            None => panic!("not found current token"),
        };
        let statement = ReturnStatement {
            token,
            value: ExpressionType::Illegal,
        };
        self.next_token();
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token()
        }
        StatementType::ReturnStatement(statement)
    }

    fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    fn register_infix(&mut self, token_type: TokenType, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, func);
    }

    fn parse_expression_statement(&mut self) -> StatementType {
        let token = match mem::replace(&mut self.cur_token, None) {
            Some(token) => token,
            None => panic!("not found current token"),
        };
        let statement = ExpressionStatement {
            token: token,
            expression: self.parse_expression(),
        };
        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        };
        StatementType::ExpressionStatement(statement)
    }

    fn parse_expression(&mut self) -> ExpressionType {
        unimplemented!();
    }
}

enum OperatorPriority {
    LOWEST = 1,
    EQUALS = 2,      // ==
    LESSGREATER = 3, // > or <
    SUM = 4,         // +
    PRODUCT = 5,     // *
    PREFIX = 6,      // -X or !X
    CALL = 7,        // myFunction(X)
}

fn discover_token_type(token: &Option<Box<Token>>) -> TokenType {
    token.clone().map_or(TokenType::Illegal, |v| v.token_type)
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
        check_parse_errors(&parser);

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

    // check errors stored in the parser struct
    fn check_parse_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.len() == 0 {
            return;
        }
        println!("parser has {} errors", errors.len());
        for message in errors {
            println!("parser error: {}", message);
        }
    }

    #[test]
    fn return_statement() {
        let input = "
return 5;
return 10;
return 993322;
";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);

        assert_eq!(
            program.len(),
            3,
            "program does not contain 3 statements. got={}",
            program.len()
        );

        for statement in program {
            let statement = match statement {
                StatementType::ReturnStatement(v) => v,
                _ => panic!("fail"),
            };
            assert_eq!(statement.token_literal(), "return");
        }
    }

    #[test]
    fn identifer_expression() {
        let input = "foobar;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(&parser);

        assert_ne!(
            program.len(),
            1,
            "program has enough statements. got={}",
            program.len()
        );
        let expression = match program.get(0).unwrap() {
            StatementType::ExpressionStatement(statement) => &statement.expression,
            _ => std::process::exit(1),
        };
        let ident = match expression {
            ExpressionType::Identifer(ident) => ident,
            _ => std::process::exit(1),
        };
        assert_eq!(&ident.value, "foobar");
        assert_eq!(ident.token_literal(), "foobar");
    }
}
