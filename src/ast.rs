use super::token::Token;
use std::io;
use std::io::Write;

pub enum StatementType {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    Illegal,
}

impl Node for StatementType {
    fn token_literal(&self) -> String {
        match self {
            StatementType::LetStatement(statement) => statement.token_literal(),
            StatementType::ReturnStatement(statement) => statement.token_literal(),
            StatementType::Illegal => String::from("Illegal"),
        }
    }
    fn string(&self) -> String {
        unimplemented!();
    }
}

impl Node for ExpressionType {
    fn token_literal(&self) -> String {
        match self {
            ExpressionType::Identifer(expression) => expression.token_literal(),
            ExpressionType::Illegal => String::from("Illegal"),
        }
    }
    fn string(&self) -> String {
        unimplemented!();
    }
}

pub enum ExpressionType {
    Identifer(Identifer),
    Illegal,
}

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub type Program = Vec<StatementType>;

impl Node for Program {
    fn token_literal(&self) -> String {
        self.get(0).map_or(String::new(), Node::token_literal)
    }
    fn string(&self) -> String {
        let mut buf = String::new();
        for v in self {
            buf.push_str(&v.string())
        }
        buf
    }
}

pub struct LetStatement {
    pub token: Box<Token>, // LET token
    pub name: Identifer,
    pub value: ExpressionType,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
    fn string(&self) -> String {
        let mut buf = String::new();

        buf.push_str(&format!("{} ", self.token_literal()));
        buf.push_str(&self.name.string());
        buf.push_str(" = ");
        buf.push_str(&self.value.string());
        buf.push_str(";");
        buf
    }
}

pub struct ReturnStatement {
    pub token: Box<Token>,
    pub value: ExpressionType,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
    fn string(&self) -> String {
        let mut buf = String::new();

        buf.push_str(&format!("{} ", self.token_literal()));
        buf.push_str(&self.value.string());
        buf.push_str(";");
        buf
    }
}

// x + 10; is valid in monkey
pub struct ExpressionStatement {
    pub token: Box<Token>,
    pub expression: ExpressionType,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
    fn string(&self) -> String {
        self.expression.string()
    }
}

#[derive(Clone)]
pub struct Identifer {
    pub token: Box<Token>, // Ident token
    pub value: String,
}

impl Identifer {
    pub fn new(token: Box<Token>) -> Self {
        let value = token.literal.clone();
        Self { token, value }
    }
}

impl Node for Identifer {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
    fn string(&self) -> String {
        self.value.clone()
    }
}
