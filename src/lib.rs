#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    ILLEGAL,
    EOF,

    // identifieres + literals
    INDENT,
    INT,

    // operators
    ASSIGN,
    PLUS,

    // delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,

    // keysords
    LBRACE,
    RBRACE,
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
            _ => Self::new_token(TokenType::EOF, &self.ch),
        };

        self.read_char();
        token
    }

    fn new_token(ttype: TokenType, literal: &String) -> Token {
        Token { ttype, literal: literal.to_owned() }
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

    // #[test]
//     fn next_token2() {
//         let input = String::from("
// let five = 5;
// let ten = 10;
//
// let add = fn(x, y) {
//     x + y;
// };
// let result = add(five, ten);
//         ");
//
//         let tokens = vec![
//             Token { ttype: TokenType::LET, literal: Some('let') },
//             Token { ttype: TokenType::IDENT, literal: Some('five') },
//             Token { ttype: TokenType::ASSIGN, literal: Some('=') },
//             Token { ttype: TokenType::INT, literal: Some('5') },
//             Token { ttype: TokenType::SEMICOLON, literal: Some(';') },
//             Token { ttype: TokenType::LET, literal: Some('let') },
//             Token { ttype: TokenType::IDENT, literal: Some('ten') },
//             Token { ttype: TokenType::ASSIGN, literal: Some('=') },
//             Token { ttype: TokenType::INT, literal: Some('10') },
//             Token { ttype: TokenType::SEMICOLON, literal: Some(';') },
//             Token { ttype: TokenType::LET, literal: Some('let') },
//             Token { ttype: TokenType::IDENT, literal: Some('add') },
//             Token { ttype: TokenType::ASSIGN, literal: Some('=') },
//             Token { ttype: TokenType::FUNCTION, literal: Some('fn') },
//             Token { ttype: TokenType::LPAREN, literal: Some('(') },
//             Token { ttype: TokenType::IDENT, literal: Some('x') },
//             Token { ttype: TokenType::COMMA, literal: Some(',') },
//             Token { ttype: TokenType::IDENT, literal: Some('y') },
//             Token { ttype: TokenType::RPAREN, literal: Some(')') },
//             Token { ttype: TokenType::LBRACE, literal: Some('{') },
//             Token { ttype: TokenType::IDENT, literal: Some('x')},
//             Token { ttype: TokenType::PLUS, literal: Some('+') },
//             Token { ttype: TokenType::IDENT, literal: Some('y') },
//             Token { ttype: TokenType::SEMICOLON, literal: Some(';') },
//             Token { ttype: TokenType::RBRACE, literal: Some('}') },
//             Token { ttype: TokenType::SEMICOLON, literal: Some(';') },
//             Token { ttype: TokenType::LET, literal: Some('let') },
//             Token { ttype: TokenType::IDENT, literal: Some('result') },
//             Token { ttype: TokenType::ASSIGN, literal: Some('=') },
//             Token { ttype: TokenType::IDENT, literal: Some('add') },
//             Token { ttype: TokenType::LPAREN, literal: Some('(') },
//             Token { ttype: TokenType::IDENT, literal: Some('five') },
//             Token { ttype: TokenType::COMMA, literal: Some(',') },
//             Token { ttype: TokenType::IDENT, literal: Some('ten') },
//             Token { ttype: TokenType::RPAREN, literal: Some(')') },
//             Token { ttype: TokenType::SEMICOLON, literal: Some(';') },
//             Token { ttype: TokenType::EOF, literal: None }, 
//         ];
//     }
}
