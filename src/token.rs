#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new_token_from_char(token_type: TokenType, literal: Option<char>) -> Self {
        Self {
            token_type,
            literal: literal.map_or("".to_string(), |v| v.to_string()),
        }
    }

    pub fn new_token_from_str(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: String::from(literal),
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token {
            token_type: TokenType::Illegal,
            literal: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Illegal, // UNKNOWN TOKEN OR STRING
    Eof,     // END OF FILE

    // 識別子(Identifer) + literal
    Ident,
    Int,

    // 演算子(operator)
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Equal,
    NotEqual,

    // delimiter
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // keyword
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl TokenType {
    pub fn value(&self) -> &'static str {
        use TokenType::*;
        match self {
            Illegal => "ILLEGAL",
            Eof => "EOF",
            Ident => "IDENT",
            Int => "INT",
            Assign => "=",
            Plus => "+",
            Minus => "-",
            Bang => "!",
            Asterisk => "*",
            Slash => "/",
            Lt => "<",
            Gt => ">",
            Equal => "==",
            NotEqual => "!=",
            Comma => ",",
            Semicolon => ";",
            Lparen => "(",
            Rparen => ")",
            Lbrace => "{",
            Rbrace => "}",
            Function => "FUNCTION",
            Let => "LET",
            True => "TRUE",
            False => "FALSE",
            If => "IF",
            Else => "ELSE",
            Return => "RETURN",
        }
    }

    pub fn lookup_iden(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Ident,
        }
    }
}
