use super::token::Token;

pub enum StatementType {
    LetStatement(LetStatement),
}

impl Node for StatementType {
    fn token_literal(&self) -> String {
        match self {
            StatementType::LetStatement(statement) => statement.token_literal(),
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
    token: Token, // LET token
    name: Identifer,
    value: ExpressionType,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}

pub struct Identifer {
    token: Token, // Ident token
    value: String,
}

impl Node for Identifer {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}
