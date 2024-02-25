use crate::tokens::Token;

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
                    Token::Eq
                } else {
                    Token::Assign
                }
            },
            ";" => Token::SemiColon,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "," => Token::Comma,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Asterisk,
            "!" => {
                if Self::next_char_is_eq(&self) {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            "/" => Token::Slash,
            "<" => Token::Lt,
            ">" => Token::Gt,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            "" => Token::Eof,
            _ => {
                if Self::is_letter(&self.ch) {
                    let literal = self.read_identifier();
                    Token::lookup_ident(&literal)

                } else if Self::is_digit(&self.ch) {
                    let literal = self.read_number();
                    Token::Int(literal.into())
                } else {
                    Token::Illegal(self.ch.to_string())
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

    pub fn read_number(&mut self) -> i32 {
        let position = self.position;

        while Self::is_digit(&self.input[self.read_position]) {
            self.read_char();
        };
        self.input[position..self.read_position].join("").parse::<i32>().unwrap()
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
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Bang,
            Token::Comma,
            Token::SemiColon,
            Token::Eof,
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
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::SemiColon,
            Token::RBrace,
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::SemiColon,
            Token::Eof, 
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
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::SemiColon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::SemiColon,
            Token::Eof, 
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
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::SemiColon,
            Token::RBrace,
            Token::Eof, 
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
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::SemiColon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::SemiColon,
            Token::Eof,
        ];

        let mut l = Lexer::new(&input);

        for token in tokens {
            let tok = l.next_token();
            println!("{:?}", tok);
            assert_eq!(tok, token);
        }
    }
}
