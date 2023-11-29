use crate::tokens::{Token, TokenType};
use crate::lexer::Lexer;
use crate::ast;

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut l: Lexer) -> Parser {
        let cur_token = l.next_token();
        let peek_token = l.next_token();

        Parser { lexer: l, cur_token, peek_token }
    } 

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(mut self) -> ast::Program {
        let mut statements: Vec<Box<dyn ast::Node>> = vec![];

        while self.cur_token.ttype != TokenType::Eof {
            let statement = self.parse_statement();
            
            if let Some(s) = statement { statements.push(s) };

            self.next_token();
        }

        ast::Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Node>> {
        let statement = match self.cur_token.ttype {
            TokenType::Let => self.parse_let_statement(),
            _ => None
        };

        if let Some(s) = statement { Some(Box::new(s)) } else { None }
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(&TokenType::Ident) {
            return None
        };

        let name = ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(&TokenType::Assign) {
            return None
        }

        self.next_token();

        // TODO: parse expression

        while self.cur_token.ttype != TokenType::SemiColon {
            self.next_token();
        };
        
        Some(ast::LetStatement { token, name })
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn let_statement() {
        let input = String::from("\
let x = 5;
let y = 10;
let foobar = 838383;");

        let l = Lexer::new(&input);
        let p = Parser::new(l);
        let program = p.parse_program();

        let n = program.statements.len();
        assert_eq!(n, 3, "Let statement must have 3 elements, got {}", n);

        let expected_identifiers = vec![
            String::from("x"),
            String::from("y"),
            String::from("fooar"),
        ];

        let ident_iter = expected_identifiers
            .iter()
            .zip(program.statements.iter());

        for (expected, statement) in ident_iter {

            let token_literal = statement.token_literal();
            assert_eq!(token_literal, String::from("let"), "Token literal not \"let\" got {}", token_literal);
            
            let name = &statement.name_token_literal();
            assert_eq!(expected, name, "Expected token {}, got {}", expected, name);
        }
    }
}
