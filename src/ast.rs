use super::token::Token;

pub enum StatementType {
    LetStatement(LetStatement),
    Illegal,
}

impl Node for StatementType {
    fn token_literal(&self) -> String {
        match self {
            StatementType::LetStatement(statement) => statement.token_literal(),
            StatementType::Illegal => String::from("Illegal"),
        }
    }
}

impl Node for ExpressionType {
    fn token_literal(&self) -> String {
        match self {
            ExpressionType::Identifer(expression) => expression.token_literal(),
        }
    }
}

pub enum ExpressionType {
    Identifer(Identifer),
}

pub trait Node {
    fn token_literal(&self) -> String;
}

pub type Program = Vec<StatementType>;

impl Node for Program {
    fn token_literal(&self) -> String {
        self.get(0).map_or(String::new(), Node::token_literal)
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
}

pub struct ReturnStatement {
    pub token: Box<Token>,
    pub return_value: ExpressionType,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}

#[derive(Clone)]
pub struct Identifer {
    pub token: Box<Token>, // Ident token
    pub value: String,
}

impl Node for Identifer {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}
