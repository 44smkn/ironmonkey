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

struct LetStatement<'a> {
    token: Token, // LET token
    name: &'a Identifer,
    value: Node,
}

impl<'a> Statement for LetStatement<'a> {}
impl<'a> Node for LetStatement<'a> {
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
