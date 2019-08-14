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

    // delimiter
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keyword
    FUNCTION,
    LET,
}

impl TokenType {
    fn value(&self) -> String {
        use TokenType::*;
        String::from( match self {
            ILLEGAL   => "ILLEGAL",
            EOF       => "EOF",
            IDENT     => "IDENT",
            INT       => "INT",
            ASSIGN    => "=",
            PLUS      => "+",
            MINUS     => "-",
            BANG      => "!",
            ASTERRISK => "*",
            SLASH     => "/",
            COMMA     => ",",
            SEMICOLON => ";",
            LPAREN    => "(",
            RPAREN    => ")",
            LBRACE    => "{",
            RBRACE    => "}",
            FUNCTION  => "FUNCTION",
            LET       => "LET",
        })
    }
}

