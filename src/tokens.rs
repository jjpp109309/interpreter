#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Illegal(String),
    Eof,

    // identifieres + literals
    Ident(String),
    Int(String),

    // operators
    Assign(String),
    Plus(String),
    Minus(String),
    Bang(String),
    Asterisk(String),
    Slash(String),
    Lt(String),
    Gt(String),

    // delimiters
    Comma(String),
    SemiColon(String),
    LParen(String),
    RParen(String),
    LBrace(String),
    RBrace(String),

    // keywords
    Function(String),
    Let(String),
    True(String),
    False(String),
    If(String),
    Else(String),
    Return(String),
    Eq(String),
    NotEq(String),
}

impl Token {
    pub fn lookup_ident(string: &String) -> Token {
        let literal = string.to_string();
        match string.as_str() {
            "fn" => Self::Function(literal),
            "let" => Self::Let(literal),
            "true" => Self::True(literal),
            "false" => Self::False(literal),
            "if" => Self::If(literal),
            "else" => Self::Else(literal),
            "return" => Self::Return(literal),
            _ => Self::Ident(literal),
        }
    }

    pub fn string(&self) -> String {
        match self {
            Token::Illegal(s) => s.to_string(),
            Token::Eof => "".to_string(),
            Token::Ident(s) => s.to_string(),
            Token::Int(s) => s.to_string(),
            Token::Assign(s) => s.to_string(),
            Token::Plus(s) => s.to_string(),
            Token::Minus(s) => s.to_string(),
            Token::Bang(s) => s.to_string(),
            Token::Asterisk(s) => s.to_string(),
            Token::Slash(s) => s.to_string(),
            Token::Lt(s) => s.to_string(),
            Token::Gt(s) => s.to_string(),
            Token::Comma(s) => s.to_string(),
            Token::SemiColon(s) => s.to_string(),
            Token::LParen(s) => s.to_string(),
            Token::RParen(s) => s.to_string(),
            Token::LBrace(s) => s.to_string(),
            Token::RBrace(s) => s.to_string(),
            Token::Function(s) => s.to_string(),
            Token::Let(s) => s.to_string(),
            Token::True(s) => s.to_string(),
            Token::False(s) => s.to_string(),
            Token::If(s) => s.to_string(),
            Token::Else(s) => s.to_string(),
            Token::Return(s) => s.to_string(),
            Token::Eq(s) => s.to_string(),
            Token::NotEq(s) => s.to_string(),
        }
    }
}
