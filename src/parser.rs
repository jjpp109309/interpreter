use crate::lexer::Lexer;
use crate::ast;
use crate::tokens::{Token, TokenType};

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser { lexer, cur_token, peek_token }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut statements: Vec<ast::Statement> = vec![];

        while self.cur_token.ttype != TokenType::Eof {
            let statement = self.parse_statement();

            if let Some(s) = statement { statements.push(s) }

            self.next_token();
        }

        ast::Program { statements }
    }
    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.cur_token.ttype {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Illegal => todo!(),
            TokenType::Eof => todo!(),
            TokenType::Ident => todo!(),
            TokenType::Int => todo!(),
            TokenType::Assign => todo!(),
            TokenType::Plus => todo!(),
            TokenType::Minus => todo!(),
            TokenType::Bang => todo!(),
            TokenType::Asterisk => todo!(),
            TokenType::Slash => todo!(),
            TokenType::Lt => todo!(),
            TokenType::Gt => todo!(),
            TokenType::Comma => todo!(),
            TokenType::SemiColon => todo!(),
            TokenType::LParen => todo!(),
            TokenType::RParen => todo!(),
            TokenType::LBrace => todo!(),
            TokenType::RBrace => todo!(),
            TokenType::Function => todo!(),
            TokenType::True => todo!(),
            TokenType::False => todo!(),
            TokenType::If => todo!(),
            TokenType::Else => todo!(),
            TokenType::Return => todo!(),
            TokenType::Eq => todo!(),
            TokenType::NotEq => todo!(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let token = self.cur_token.clone();

        if !self.expect_peek() {
            return None
        };

        let name = ast::Identifier{
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone()
        };
        
        if !self.expect_peek() {
            return None
        };
        
        self.next_token();

        let value = self.parse_expression();
        
        Some(ast::Statement { token, name, value })
    }

    fn expect_peek(&mut self) -> bool {
        match &self.peek_token.ttype {
            TokenType::Let => {
                self.next_token();
                true
            },
            _ => false
        }
    }

    fn parse_expression(&self) -> ast::Expression {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ast::*;
    
    #[test]
    fn let_statement() {
        let input = String::from("\
let x = 5;
let y = 10;
let foobar = 838383;");
        let l = Lexer::new(&input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        
        let n = program.statements.len();
        assert_eq!(n, 3, "Let statements must have 3 elements. Got {}", n);
        
        let expected_identifiers = vec![
            String::from("x"),
            String::from("y"),
            String::from("foobar"),
        ];

        let ident_iter = expected_identifiers
            .iter()
            .zip(program.statements.iter());

        for (expected, statement) in ident_iter {
            let token_literal = statement.token_literal();
            println!("{:?}", token_literal);
            assert_eq!(
                token_literal,
                String::from("let"),
                "Token literal not \"let\" got {}",
                token_literal
            );

            let value = &statement.name.value;
            assert_eq!(
                value,
                expected,
                "Identifier value doesn't match. Got {}",
                value
            );

            let ident_literal = &statement.name.token_literal();
            assert_eq!(
                ident_literal,
                expected,
                "Identifier literal doesn't match. Got {}",
                ident_literal
            )
        }
    }
}
