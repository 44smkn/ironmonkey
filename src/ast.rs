use super::token::Token;

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
}

impl Node for ExpressionType {
    fn token_literal(&self) -> String {
        match self {
            ExpressionType::Identifer(expression) => expression.token_literal(),
            ExpressionType::Illegal => String::from("Illegal"),
        }
    }
}

pub enum ExpressionType {
    Identifer(Identifer),
    Illegal,
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
    pub value: ExpressionType,
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
}
