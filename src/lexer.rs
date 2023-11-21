#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,

    // identifieres + literals
    Ident,
    Int,

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

impl TokenType {
    fn lookup_ident(literal: &String) -> TokenType {
        match literal.as_str() {
            "fn" => Self::Function,
            "let" => Self::Let,
            "true" => Self::True,
            "false" => Self::False,
            "if" => Self::If,
            "else" => Self::Else,
            "return" => Self::Return,
            _ => Self::Ident
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
}

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<String>,
    pub position: usize,
    pub read_position: usize,
    pub ch: String,
}

impl Lexer {
    pub fn new(input: &String) -> Self {
        let input_chars = input
            .chars()
            .map(|char| char.to_string())
            .collect();

        let mut l = Lexer {
            input: input_chars,
            position: 0,
            read_position: 0,
            ch: String::from(""),
        };
        
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len().try_into().unwrap() {
            self.ch = String::from("");
        } else {
            self.ch = self.input[self.read_position].to_owned();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();

        let character = self.ch.as_str();
        let token = match character {
            "=" => {
                if Self::next_char_is_eq(&self) {
                    self.read_char();
                    let literal = String::from("==");
                    Self::new_token(TokenType::Eq, &literal)
                } else {
                    Self::new_token(TokenType::Assign, &self.ch)
                }
            },
            ";" => Self::new_token(TokenType::SemiColon, &self.ch),
            "(" => Self::new_token(TokenType::LParen, &self.ch),
            ")" => Self::new_token(TokenType::RParen, &self.ch),
            "," => Self::new_token(TokenType::Comma, &self.ch),
            "+" => Self::new_token(TokenType::Plus, &self.ch),
            "-" => Self::new_token(TokenType::Minus, &self.ch),
            "*" => Self::new_token(TokenType::Asterisk, &self.ch),
            "!" => {
                if Self::next_char_is_eq(&self) {
                    self.read_char();
                    let literal = String::from("!=");
                    Self::new_token(TokenType::NotEq, &literal)
                } else {
                    Self::new_token(TokenType::Bang, &self.ch)
                }
            }
            "/" => Self::new_token(TokenType::Slash, &self.ch),
            "<" => Self::new_token(TokenType::Lt, &self.ch),
            ">" => Self::new_token(TokenType::Gt, &self.ch),
            "{" => Self::new_token(TokenType::LBrace, &self.ch),
            "}" => Self::new_token(TokenType::RBrace, &self.ch),
            "" => Self::new_token(TokenType::Eof, &self.ch),
            _ => {
                if Self::is_letter(&self.ch) {
                    let literal = self.read_identifier();
                    let token_type = TokenType::lookup_ident(&literal);

                    Self::new_token(token_type, &literal)
                } else if Self::is_digit(&self.ch) {
                    let literal = self.read_number();
                    let token_type = TokenType::Int;

                    Self::new_token(token_type, &literal)

                } else {
                    Self::new_token(TokenType::Illegal, &self.ch)
                }
            }
        };

        self.read_char();
        token
    }

    pub fn skip_white_space(&mut self) {
        while " " == self.ch  || "\t" == self.ch || "\n" == self.ch || "\r" == self.ch {
            self.read_char()
        }
    }

    pub fn new_token(ttype: TokenType, literal: &String) -> Token {
        Token { ttype, literal: literal.to_owned() }
    }

    pub fn is_letter(letter: &String) -> bool {
        let ident_characters = String::from(
            "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM_"
        );

        ident_characters.contains(letter)
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;

        while Self::is_letter(&self.input[self.read_position]) {
            self.read_char();
        };

        self.input[position..self.read_position].join("")
    }

    pub fn is_digit(character: &String) -> bool {
        "1234567890".contains(character)
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;

        while Self::is_digit(&self.input[self.read_position]) {
            self.read_char();
        };
        self.input[position..self.read_position].join("")
    }

    pub fn next_char_is_eq(&self) -> bool {
        self.input[self.read_position] == "="
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn next_token() {
        let input = String::from("=+(){}!,;");
        let tokens = vec![
            Token { ttype: TokenType::Assign, literal: "=".to_string() },
            Token { ttype: TokenType::Plus, literal: "+".to_string() },
            Token { ttype: TokenType::LParen, literal: "(".to_string() },
            Token { ttype: TokenType::RParen, literal: ")".to_string() },
            Token { ttype: TokenType::LBrace, literal: "{".to_string() },
            Token { ttype: TokenType::RBrace, literal: "}".to_string() },
            Token { ttype: TokenType::Bang, literal: "!".to_string() },
            Token { ttype: TokenType::Comma, literal: ",".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::Eof, literal: "".to_string() },
        ];

        let mut l = Lexer::new(&input);

        for token in tokens {
            let tok = l.next_token();
            assert_eq!(tok, token);
        }
    }

    #[test]
    fn next_token2() {
        let input = String::from("\
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);");

        let tokens = vec![
            Token { ttype: TokenType::Let, literal: "let".to_string() },
            Token { ttype: TokenType::Ident, literal: "five".to_string() },
            Token { ttype: TokenType::Assign, literal: "=".to_string() },
            Token { ttype: TokenType::Int, literal: "5".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::Let, literal: "let".to_string() },
            Token { ttype: TokenType::Ident, literal: "ten".to_string() },
            Token { ttype: TokenType::Assign, literal: "=".to_string() },
            Token { ttype: TokenType::Int, literal: "10".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::Let, literal: "let".to_string() },
            Token { ttype: TokenType::Ident, literal: "add".to_string() },
            Token { ttype: TokenType::Assign, literal: "=".to_string() },
            Token { ttype: TokenType::Function, literal: "fn".to_string() },
            Token { ttype: TokenType::LParen, literal: "(".to_string() },
            Token { ttype: TokenType::Ident, literal: "x".to_string() },
            Token { ttype: TokenType::Comma, literal: ",".to_string() },
            Token { ttype: TokenType::Ident, literal: "y".to_string() },
            Token { ttype: TokenType::RParen, literal: ")".to_string() },
            Token { ttype: TokenType::LBrace, literal: "{".to_string() },
            Token { ttype: TokenType::Ident, literal: "x".to_string()},
            Token { ttype: TokenType::Plus, literal: "+".to_string() },
            Token { ttype: TokenType::Ident, literal: "y".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::RBrace, literal: "}".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::Let, literal: "let".to_string() },
            Token { ttype: TokenType::Ident, literal: "result".to_string() },
            Token { ttype: TokenType::Assign, literal: "=".to_string() },
            Token { ttype: TokenType::Ident, literal: "add".to_string() },
            Token { ttype: TokenType::LParen, literal: "(".to_string() },
            Token { ttype: TokenType::Ident, literal: "five".to_string() },
            Token { ttype: TokenType::Comma, literal: ",".to_string() },
            Token { ttype: TokenType::Ident, literal: "ten".to_string() },
            Token { ttype: TokenType::RParen, literal: ")".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::Eof, literal: "".to_string() }, 
        ];

        let mut l = Lexer::new(&input);

        for token in tokens {
            let tok = l.next_token();
            println!("{:?}", tok);
            assert_eq!(tok, token);
        }
    }

    #[test]
    fn next_token3() {
        let input = String::from("\
!-/*5;
5 < 10 > 5;");

        let tokens = vec![
            Token { ttype: TokenType::Bang, literal: "!".to_string() },
            Token { ttype: TokenType::Minus, literal: "-".to_string() },
            Token { ttype: TokenType::Slash, literal: "/".to_string() },
            Token { ttype: TokenType::Asterisk, literal: "*".to_string() },
            Token { ttype: TokenType::Int, literal: "5".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::Int, literal: "5".to_string() },
            Token { ttype: TokenType::Lt, literal: "<".to_string() },
            Token { ttype: TokenType::Int, literal: "10".to_string() },
            Token { ttype: TokenType::Gt, literal: ">".to_string() },
            Token { ttype: TokenType::Int, literal: "5".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::Eof, literal: "".to_string() }, 
        ];

        let mut l = Lexer::new(&input);

        for token in tokens {
            let tok = l.next_token();
            println!("{:?}", tok);
            assert_eq!(tok, token);
        }
    }

    #[test]
    fn next_token4() {
    let input = String::from("\
if (5 < 10) {
    return true;
} else {
    return false;
}");

        let tokens = vec![
            Token { ttype: TokenType::If, literal: "if".to_string() },
            Token { ttype: TokenType::LParen, literal: "(".to_string() },
            Token { ttype: TokenType::Int, literal: "5".to_string() },
            Token { ttype: TokenType::Lt, literal: "<".to_string() },
            Token { ttype: TokenType::Int, literal: "10".to_string() },
            Token { ttype: TokenType::RParen, literal: ")".to_string() },
            Token { ttype: TokenType::LBrace, literal: "{".to_string() },
            Token { ttype: TokenType::Return, literal: "return".to_string() },
            Token { ttype: TokenType::True, literal: "true".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::RBrace, literal: "}".to_string() },
            Token { ttype: TokenType::Else, literal: "else".to_string() },
            Token { ttype: TokenType::LBrace, literal: "{".to_string() },
            Token { ttype: TokenType::Return, literal: "return".to_string() },
            Token { ttype: TokenType::False, literal: "false".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::RBrace, literal: "}".to_string() },
            Token { ttype: TokenType::Eof, literal: "".to_string() }, 
        ];

        let mut l = Lexer::new(&input);

        for token in tokens {
            let tok = l.next_token();
            println!("{:?}", tok);
            assert_eq!(tok, token);
        }
    }

    #[test]
    fn next_token5() {
    let input = String::from("\
10 == 10;
10 != 9;");

        let tokens = vec![
            Token { ttype: TokenType::Int, literal: "10".to_string() },
            Token { ttype: TokenType::Eq, literal: "==".to_string() },
            Token { ttype: TokenType::Int, literal: "10".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::Int, literal: "10".to_string() },
            Token { ttype: TokenType::NotEq, literal: "!=".to_string() },
            Token { ttype: TokenType::Int, literal: "9".to_string() },
            Token { ttype: TokenType::SemiColon, literal: ";".to_string() },
            Token { ttype: TokenType::Eof, literal: "".to_string() },
        ];

        let mut l = Lexer::new(&input);

        for token in tokens {
            let tok = l.next_token();
            println!("{:?}", tok);
            assert_eq!(tok, token);
        }
    }
}
