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

struct Token {
    ttype: TokenType,
    literal: String,
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn next_token() {
        let input = String::from("=+(){},;");
        let tokens = vec![
            Token { ttype: TokenType::ASSIGN, literal: String::from("=") },
            Token { ttype: TokenType::PLUS, literal: String::from("+") },
            Token { ttype: TokenType::LPAREN, literal: String::from("(") },
            Token { ttype: TokenType::RPAREN, literal: String::from(")") },
            Token { ttype: TokenType::LBRACE, literal: String::from("{") },
            Token { ttype: TokenType::RBRACE, literal: String::from("}") },
            Token { ttype: TokenType::COMMA, literal: String::from(",") },
            Token { ttype: TokenType::SEMICOLON, literal: String::from(";") },
            Token { ttype: TokenType::EOF, literal: String::from("") },
        ];

        let l = Token::new(input);
        for token in tokens {
            let tok = l.next_token();
            assert_eq!(tok.ttype, token.ttype);
            assert_eq!(tok.literal, token.literal);
        }
    }
}
