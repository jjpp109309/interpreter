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

            if let Some(s) = statement { statements.push(s) };

            self.next_token();
        }

        ast::Program { statements }
    }
    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.cur_token.ttype {
            TokenType::Let => self.parse_let_statement(),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(&TokenType::Ident) {
            return None
        };

        let name = ast::Identifier{
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone()
        };
        
        if !self.expect_peek(&TokenType::Assign) {
            return None
        };
        
        self.next_token();

        while self.cur_token.ttype != TokenType::SemiColon {
            self.next_token();
        }
        
        Some(ast::Statement { token, name: Some(name), value: None })
    }

    fn expect_peek(&mut self, ttype: &TokenType) -> bool {
        let peek_token = &self.peek_token.ttype;

        if ttype == peek_token {
            self.next_token();
            true
        } else {
            panic!("Expected token type to be {:?}, got {:?}", ttype, peek_token);
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
        // sucessful test
        let input = String::from("\
let x = 5;
let y = 10;
let foobar = 838383;");
        // error test
// let input = String::from("\
// let x 5;
// let = 10;
// let 838383;");

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
            assert_eq!(token_literal, String::from("let"), "Token literal not \"let\" got {}", token_literal);

            if let Some(name) = &statement.name {
                let value = &name.value;
                assert_eq!(value, expected, "Identifier value doesn't match. Got {}", value);
            }


            if let Some(name) = &statement.name {
                let ident_literal = &name.token_literal();
                assert_eq!(ident_literal, expected, "Identifier literal doesn't match. Got {}", ident_literal)
            }
        }
    }
}
