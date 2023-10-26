use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    ILLEGAL,
    EOF,

    // identifieres + literals
    IDENT,
    INT,

    // operators
    ASSIGN,
    PLUS,

    // delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keysords
    FUNCTION,
    LET,
}

impl TokenType {
    fn lookup_ident(literal: String) -> TokenType {
        match literal.as_str() {
            "fn" => Self::FUNCTION,
            "let" => Self::LET,
            _ => Self::IDENT
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Token {
    ttype: TokenType,
    literal: String,
}

struct Lexer {
    input: Vec<String>,
    position: usize,
    read_position: usize,
    ch: String,
}

impl Lexer {
    fn new(input: String) -> Self {
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

    fn read_char(&mut self) {
        if self.read_position >= self.input.len().try_into().unwrap() {
            self.ch = String::from("");
        } else {
            self.ch = self.input[self.read_position].to_owned();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let character = self.ch.as_str();
        let token = match character {
            "=" => Self::new_token(TokenType::ASSIGN, &self.ch),
            ";" => Self::new_token(TokenType::SEMICOLON, &self.ch),
            "(" => Self::new_token(TokenType::LPAREN, &self.ch),
            ")" => Self::new_token(TokenType::RPAREN, &self.ch),
            "," => Self::new_token(TokenType::COMMA, &self.ch),
            "+" => Self::new_token(TokenType::PLUS, &self.ch),
            "{" => Self::new_token(TokenType::LBRACE, &self.ch),
            "}" => Self::new_token(TokenType::RBRACE, &self.ch),
            "" => Self::new_token(TokenType::EOF, &self.ch),
            _ => {
                if Self::is_letter(&self.ch) {
                    let literal = self.get_identifier();
                    let token_type = TokenType::lookup_ident(literal);

                    Self::new_token(TokenType::IDENT, &literal)
                } else {
                    Self::new_token(TokenType::ILLEGAL, &self.ch)
                }
            }
        };

        self.read_char();
        token
    }

    fn new_token(ttype: TokenType, literal: &String) -> Token {
        Token { ttype, literal: literal.to_owned() }
    }

    fn is_letter(letter: &String) -> bool {
        let ident_characters = String::from(
            "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM_"
        );

        ident_characters.contains(letter)
    }

    fn get_identifier(&mut self) -> String {
        let position = self.position;
        let character = self.ch.to_string();
        while Self::is_letter(&character) {
            self.read_char();
        };

        return self.input[position..self.position].join("")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn next_token() {
        let input = String::from("=+(){},;");
        let tokens = vec![
            Token { ttype: TokenType::ASSIGN, literal: "=".to_string() },
            Token { ttype: TokenType::PLUS, literal: "+".to_string() },
            Token { ttype: TokenType::LPAREN, literal: "(".to_string() },
            Token { ttype: TokenType::RPAREN, literal: ")".to_string() },
            Token { ttype: TokenType::LBRACE, literal: "{".to_string() },
            Token { ttype: TokenType::RBRACE, literal: "}".to_string() },
            Token { ttype: TokenType::COMMA, literal: ",".to_string() },
            Token { ttype: TokenType::SEMICOLON, literal: ";".to_string() },
            Token { ttype: TokenType::EOF, literal: "".to_string() },
        ];

        let mut l = Lexer::new(input);

        for token in tokens {
            let tok = l.next_token();
            assert_eq!(tok, token);
        }
    }

    #[test]
    fn next_token2() {
        let input = String::from("
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
        ");

        let tokens = vec![
            Token { ttype: TokenType::LET, literal: "let".to_string() },
            Token { ttype: TokenType::IDENT, literal: "five".to_string() },
            Token { ttype: TokenType::ASSIGN, literal: "=".to_string() },
            Token { ttype: TokenType::INT, literal: "5".to_string() },
            Token { ttype: TokenType::SEMICOLON, literal: ";".to_string() },
            Token { ttype: TokenType::LET, literal: "let".to_string() },
            Token { ttype: TokenType::IDENT, literal: "ten".to_string() },
            Token { ttype: TokenType::ASSIGN, literal: "=".to_string() },
            Token { ttype: TokenType::INT, literal: "10".to_string() },
            Token { ttype: TokenType::SEMICOLON, literal: ";".to_string() },
            Token { ttype: TokenType::LET, literal: "let".to_string() },
            Token { ttype: TokenType::IDENT, literal: "add".to_string() },
            Token { ttype: TokenType::ASSIGN, literal: "=".to_string() },
            Token { ttype: TokenType::FUNCTION, literal: "fn".to_string() },
            Token { ttype: TokenType::LPAREN, literal: "(".to_string() },
            Token { ttype: TokenType::IDENT, literal: "x".to_string() },
            Token { ttype: TokenType::COMMA, literal: ",".to_string() },
            Token { ttype: TokenType::IDENT, literal: "y".to_string() },
            Token { ttype: TokenType::RPAREN, literal: ")".to_string() },
            Token { ttype: TokenType::LBRACE, literal: "{".to_string() },
            Token { ttype: TokenType::IDENT, literal: "x".to_string()},
            Token { ttype: TokenType::PLUS, literal: "+".to_string() },
            Token { ttype: TokenType::IDENT, literal: "y".to_string() },
            Token { ttype: TokenType::SEMICOLON, literal: ";".to_string() },
            Token { ttype: TokenType::RBRACE, literal: "}".to_string() },
            Token { ttype: TokenType::SEMICOLON, literal: ";".to_string() },
            Token { ttype: TokenType::LET, literal: "let".to_string() },
            Token { ttype: TokenType::IDENT, literal: "result".to_string() },
            Token { ttype: TokenType::ASSIGN, literal: "=".to_string() },
            Token { ttype: TokenType::IDENT, literal: "add".to_string() },
            Token { ttype: TokenType::LPAREN, literal: "(".to_string() },
            Token { ttype: TokenType::IDENT, literal: "five".to_string() },
            Token { ttype: TokenType::COMMA, literal: ",".to_string() },
            Token { ttype: TokenType::IDENT, literal: "ten".to_string() },
            Token { ttype: TokenType::RPAREN, literal: ")".to_string() },
            Token { ttype: TokenType::SEMICOLON, literal: ";".to_string() },
            Token { ttype: TokenType::EOF, literal: "".to_string() }, 
        ];
    }
}
