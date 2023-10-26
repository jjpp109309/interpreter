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

#[derive(PartialEq, Eq)]
struct Token {
    ttype: TokenType,
    literal: Option<char>,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let input_chars = input.chars().collect();
        let mut l = Lexer {
            input: input_chars,
            position: 0,
            read_position: 0,
            ch: None,
        };
        
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len().try_into().unwrap() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some('=') => Token { ttype: TokenType::ASSIGN, literal: self.ch },
            Some(';') => Token { ttype: TokenType::SEMICOLON, literal: self.ch },
            Some('(') => Token { ttype: TokenType::LPAREN, literal: self.ch },
            Some(')') => Token { ttype: TokenType::RPAREN, literal: self.ch },
            Some(',') => Token { ttype: TokenType::COMMA, literal: self.ch },
            Some('+') => Token { ttype: TokenType::PLUS, literal: self.ch },
            Some('{') => Token { ttype: TokenType::LBRACE, literal: self.ch },
            Some('}') => Token { ttype: TokenType::RBRACE, literal: self.ch },
            _ => Token { ttype: TokenType::EOF, literal: None},
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn next_token() {
        let input = String::from("=+(){},;");
        let tokens = vec![
            Token { ttype: TokenType::ASSIGN, literal: Some('=') },
            Token { ttype: TokenType::PLUS, literal: Some('+') },
            Token { ttype: TokenType::LPAREN, literal: Some('(') },
            Token { ttype: TokenType::RPAREN, literal: Some(')') },
            Token { ttype: TokenType::LBRACE, literal: Some('{') },
            Token { ttype: TokenType::RBRACE, literal: Some('}') },
            Token { ttype: TokenType::COMMA, literal: Some(',') },
            Token { ttype: TokenType::SEMICOLON, literal: Some(';') },
            Token { ttype: TokenType::EOF, literal: None },
        ];

        let mut l = Lexer::new(input);

        for token in tokens {
            let tok = l.next_token();
            println!("{:?}", tok.literal);
            assert_eq!(tok.ttype, token.ttype);
            assert_eq!(tok.literal, token.literal);
        }
    }
}
