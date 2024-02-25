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
            Token::Eof => "".to_string(),
            Token::Ident(s) => s.to_string(),
            Token::Int(s) => s.to_string(),
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
