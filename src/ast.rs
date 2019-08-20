use super::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement {}
pub trait Expression {}

pub struct Program<T: Node> {
    statements: Vec<T>,
}

impl<T: Node> Node for Program<T> {
    fn token_literal(&self) -> String {
        self.statements
            .get(0)
            .map_or(String::new(), Node::token_literal)
    }
}

pub struct LetStatement<T: Expression> {
    token: Token, // LET token
    name: Identifer,
    value: T,
}

impl<T: Expression> Statement for LetStatement<T> {}
impl<T: Expression> Node for LetStatement<T> {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}

pub struct Identifer {
    token: Token, // Ident token
    value: String,
}

impl Expression for Identifer {}
impl Node for Identifer {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}
