use super::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

trait Statement {}
trait Expression {}

pub struct Program<T: Node> {
    statements: Vec<T>,
}

impl<T: Node> Node for Program<T> {
    fn token_literal(&self) -> String {
        self.statements
            .get(0)
            .map_or(String::new(), |v| v.token_literal())
    }
}

pub struct LetStatement {
    token: Token, // LET token
    name: Identifer,
    value: Node,
}

impl Statement for LetStatement {}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}

struct Identifer {
    token: Token, // Ident token
    value: String,
}

impl Expression for Identifer {}
impl Node for Identifer {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}
