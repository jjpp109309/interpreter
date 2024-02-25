#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Token {
    Illegal(String),
    Eof,

    // identifieres + literals
    Ident(String),
    Int(i32),

    // operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,

    // delimiters
    Comma,
    SemiColon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
}

impl Token {
    pub fn lookup_ident(string: &String) -> Token {
        let literal = string.to_string();
        match string.as_str() {
            "fn" => Self::Function,
            "let" => Self::Let,
            "true" => Self::True,
            "false" => Self::False,
            "if" => Self::If,
            "else" => Self::Else,
            "return" => Self::Return,
            _ => Self::Ident(literal),
        }
    }

    pub fn string(&self) -> String {
        match self {
            Token::Illegal(s) => s.to_string(),
            Token::Ident(s) => s.to_string(),
            Token::Int(s) => s.to_string(),
            Token::Eof => Self::name(&Token::Eof),
            Token::Assign => Self::name(&Token::Assign),
            Token::Plus => Self::name(&Token::Plus),
            Token::Minus => Self::name(&Token::Minus),
            Token::Bang => Self::name(&Token::Bang),
            Token::Asterisk => Self::name(&Token::Asterisk),
            Token::Slash => Self::name(&Token::Slash),
            Token::Lt => Self::name(&Token::Lt),
            Token::Gt => Self::name(&Token::Gt),
            Token::Comma => Self::name(&Token::Comma),
            Token::SemiColon => Self::name(&Token::SemiColon),
            Token::LParen => Self::name(&Token::LParen),
            Token::RParen => Self::name(&Token::RParen),
            Token::LBrace => Self::name(&Token::LBrace),
            Token::RBrace => Self::name(&Token::RBrace),
            Token::Function => Self::name(&Token::Function),
            Token::Let => Self::name(&Token::Let),
            Token::True => Self::name(&Token::True),
            Token::False => Self::name(&Token::False),
            Token::If => Self::name(&Token::If),
            Token::Else => Self::name(&Token::Else),
            Token::Return => Self::name(&Token::Return),
            Token::Eq => Self::name(&Token::Eq),
            Token::NotEq => Self::name(&Token::NotEq),
        }
    }
    
    pub fn name(token: &Token) -> String {
        match token {
            Token::Illegal(s) => "illegal".to_string(),
            Token::Ident(s) => "identifier".to_string(),
            Token::Int(s) => "integer".to_string(),
            Token::Eof => "".to_string(),
            Token::Assign => "=".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Bang => "!".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Lt => "<".to_string(),
            Token::Gt => ">".to_string(),
            Token::Comma => ",".to_string(),
            Token::SemiColon => ";".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => "".to_string(),
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::Function => "fn".to_string(),
            Token::Let => "let".to_string(),
            Token::True => "true".to_string(),
            Token::False => "false".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::Return => "return".to_string(),
            Token::Eq => "==".to_string(),
            Token::NotEq => "!=".to_string(),
        }
    }
}
