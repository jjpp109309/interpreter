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


