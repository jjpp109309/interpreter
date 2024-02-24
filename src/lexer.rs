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
                    Token::Eq("==".to_string())
                } else {
                    Token::Assign("=".to_string())
                }
            },
            ";" => Token::SemiColon(";".to_string()),
            "(" => Token::LParen("(".to_string()),
            ")" => Token::RParen(")".to_string()),
            "," => Token::Comma(",".to_string()),
            "+" => Token::Plus("+".to_string()),
            "-" => Token::Minus("-".to_string()),
            "*" => Token::Asterisk("*".to_string()),
            "!" => {
                if Self::next_char_is_eq(&self) {
                    self.read_char();
                    Token::NotEq("!=".to_string())
                } else {
                    Token::Bang("!".to_string())
                }
            }
            "/" => Token::Slash("/".to_string()),
            "<" => Token::Lt("<".to_string()),
            ">" => Token::Gt(">".to_string()),
            "{" => Token::LBrace("{".to_string()),
            "}" => Token::RBrace("}".to_string()),
            "" => Token::Eof,
            _ => {
                if Self::is_letter(&self.ch) {
                    let literal = self.read_identifier();
                    Token::lookup_ident(&literal)

                } else if Self::is_digit(&self.ch) {
                    let literal = self.read_number();
                    Token::Int(literal)
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
            Token::Assign("=".to_string()),
            Token::Plus("+".to_string()),
            Token::LParen("(".to_string()),
            Token::RParen(")".to_string()),
            Token::LBrace("{".to_string()),
            Token::RBrace("}".to_string()),
            Token::Bang("!".to_string()),
            Token::Comma(",".to_string()),
            Token::SemiColon(";".to_string()),
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
            Token::Let("let".to_string()),
            Token::Ident(String::from("five")),
            Token::Assign("=".to_string()),
            Token::Int(String::from("5")),
            Token::SemiColon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Ident(String::from("ten")),
            Token::Assign("=".to_string()),
            Token::Int(String::from("10")),
            Token::SemiColon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Ident(String::from("add")),
            Token::Assign("=".to_string()),
            Token::Function("fn".to_string()),
            Token::LParen("(".to_string()),
            Token::Ident(String::from("x")),
            Token::Comma(",".to_string()),
            Token::Ident(String::from("y")),
            Token::RParen(")".to_string()),
            Token::LBrace("{".to_string()),
            Token::Ident(String::from("x")),
            Token::Plus("+".to_string()),
            Token::Ident(String::from("y")),
            Token::SemiColon(";".to_string()),
            Token::RBrace("}".to_string()),
            Token::SemiColon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Ident(String::from("result")),
            Token::Assign("=".to_string()),
            Token::Ident(String::from("add")),
            Token::LParen("(".to_string()),
            Token::Ident(String::from("five")),
            Token::Comma(",".to_string()),
            Token::Ident(String::from("ten")),
            Token::RParen(")".to_string()),
            Token::SemiColon(";".to_string()),
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
            Token::Bang("!".to_string()),
            Token::Minus("-".to_string()),
            Token::Slash("/".to_string()),
            Token::Asterisk("*".to_string()),
            Token::Int(String::from("5")),
            Token::SemiColon(";".to_string()),
            Token::Int(String::from("5")),
            Token::Lt("<".to_string()),
            Token::Int(String::from("10")),
            Token::Gt(">".to_string()),
            Token::Int(String::from("5")),
            Token::SemiColon(";".to_string()),
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
            Token::If("if".to_string()),
            Token::LParen("(".to_string()),
            Token::Int(String::from("5")),
            Token::Lt("<".to_string()),
            Token::Int(String::from("10")),
            Token::RParen(")".to_string()),
            Token::LBrace("{".to_string()),
            Token::Return("return".to_string()),
            Token::True("true".to_string()),
            Token::SemiColon(";".to_string()),
            Token::RBrace("}".to_string()),
            Token::Else("else".to_string()),
            Token::LBrace("{".to_string()),
            Token::Return("return".to_string()),
            Token::False("false".to_string()),
            Token::SemiColon(";".to_string()),
            Token::RBrace("}".to_string()),
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
            Token::Int(String::from("10")),
            Token::Eq("==".to_string()),
            Token::Int(String::from("10")),
            Token::SemiColon(";".to_string()),
            Token::Int(String::from("10")),
            Token::NotEq("!=".to_string()),
            Token::Int(String::from("9")),
            Token::SemiColon(";".to_string()),
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
