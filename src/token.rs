#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new_token_from_char(token_type: TokenType, literal: Option<char>) -> Self {
        Self {
            token_type,
            literal: literal.map_or("".to_string(), |v| v.to_string())
        }
    }

    pub fn new_token_from_str(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

#[derive(Debug)]
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
    pub fn value(&self) -> &'static str {
        use TokenType::*;
        match self {
            ILLEGAL => "ILLEGAL",
            EOF => "EOF",
            IDENT => "IDENT",
            INT => "INT",
            ASSIGN => "=",
            PLUS => "+",
            MINUS => "-",
            BANG => "!",
            ASTERRISK => "*",
            SLASH => "/",
            COMMA => ",",
            SEMICOLON => ";",
            LPAREN => "(",
            RPAREN => ")",
            LBRACE => "{",
            RBRACE => "}",
            FUNCTION => "FUNCTION",
            LET => "LET",
        }
    }

    pub fn lookup_iden(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            _ => TokenType::IDENT,
        }
    }
}
