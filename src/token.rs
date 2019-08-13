pub struct Token {
    type: TokenType,
    literal: String,
}

pub enum TokenType {
    ILLEGAL, // UNKNOWN TOKEN OR STRING
    EOF,     // END OF FILE

    // 識別子(Identifer) + literal
	IDENT,
	INT,

    // 演算子(operator)
	ASSIGN,
	PLUS,
	MINUS,
	BANG,
	ASTERRISK,
	SLASH,
}

impl TokenType {
    fn value(&self) -> String {
        match self {
            TokenType::ILLEGAL => String::from("ILLEGAL"),
            TokenType::EOF => String::from("EOF"),
            TokenType::IDENT => String::from("IDENT"),
            TokenType::ASSIGN => String::from("="),
            TokenType::PLUS => String::from("+"),
            TokenType::MINUS => String::from("-"),
            TokenType::BANG => String::from("!"),
            TokenType::ASTERRISK => String::from("*"),
            TokenType::SLASH => String::from("/"),
        }
    }
}

