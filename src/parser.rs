use crate::tokens::Token;
use crate::lexer::Lexer;
use crate::ast::{Statement, Program};
use std::collections::HashMap;
use crate::expressions::Expression;

type PrefixFn = fn() -> Expression;
type InfixFn = fn(Expression) -> Expression;

pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

impl Precedence {
    fn rank(&self) -> u8 {
        match self {
            Precedence::LOWEST => 0,
            Precedence::EQUALS => 1,
            Precedence::LESSGREATER => 2,
            Precedence::SUM => 3,
            Precedence::PRODUCT => 4,
            Precedence::PREFIX => 5,
            Precedence::CALL => 6,
        }
    }
}

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    prefix_parse_fns: HashMap<String, fn(&Token)->Token>,
    // infix_parse_fns: HashMap<String, fn()>,
}

impl Parser {
    fn new(mut l: Lexer) -> Parser {
        let cur_token = l.next_token();
        let peek_token = l.next_token();

        let prefix_parse_fns = HashMap::new();
        // let infix_parse_fns = HashMap::new();

        prefix_parse_fns.insert("identifier".to_string(), parse_identifier);

        Parser { lexer: l, cur_token, peek_token, prefix_parse_fns }
    } 

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(mut self) -> Program {
        let mut statements: Vec<Statement> = vec![];

        while self.cur_token != Token::Eof {
            let statement = self.parse_statement();

            statements.push(statement);
            self.next_token();
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Statement {
        let token = self.cur_token.clone();

        match self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => panic!("Next token not Ident")
        }

        let identifier = self.cur_token.clone();

        match self.peek_token {
            Token::Assign => self.next_token(),
            _ => panic!("Next token not Ident")
        }

        // TODO: parse expression

        loop {
            match self.cur_token {
                Token::SemiColon => break,
                _ => self.next_token(),
            }
        }

        Statement::Let { token, identifier }
    }

    fn parse_return_statement(&mut self) -> Statement {
        let token = self.cur_token.clone();

        self.next_token();

        // TODO: parse expression

        loop {
            match self.cur_token {
                Token::SemiColon => break,
                _ => self.next_token(),
            }
        }

        Statement::Return { token }
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::LOWEST);

        match self.cur_token {
            Token::SemiColon => self.next_token(),
            _ => {},
        }

        Statement::Expression { token, expression }
    }

    fn parse_expression(&self, precedence: Precedence) -> Expression {
        let token_name = Token::name(&self.cur_token)
        self.prefix_parse_fns.get(&self.cur_token).unwrap()()
    }
}

fn parse_identifier(cur_token: &Token) -> Token {
    match cur_token {
        Token::Ident(_) => cur_token.clone(),
        _ => panic!("Token is not identifier, got {:?}", cur_token),
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Node;
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
            String::from("foobar"),
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

    #[test]
    fn return_statement() {
        let input = String::from("\
return 5;
return 10;
return 3301;");

        let l = Lexer::new(&input);
        let p = Parser::new(l);
        let program = p.parse_program();

        let n = program.statements.len();
        assert_eq!(n, 3, "Return statements must have 3 elements. Got {}", n);

        for statement in program.statements {
            let token_literal = statement.token_literal();
            assert_eq!(token_literal, String::from("return"), "Token literal not \"let\" got {}", token_literal);
        }
    }

    #[test]
    fn identifier_statement() {
        let input = String::from("\
foobar;");

        let l = Lexer::new(&input);
        let p = Parser::new(l);
        let program = p.parse_program();

        let n = program.statements.len();
        assert_eq!(n, 1, "Program has not enough statements. Got {}", n);
        println!("statements {:?}", program.statements);

        let statement = &program.statements[0];

        // match statement {
        //     Statement::Expression { expression, .. } => {
        //         match expression {
        //             Token::Ident(i) => {
        //                 if i != &String::from("foobar") {
        //                     panic!("Identifier value not correct. Got {:?}", i)
        //                 }
        //             },
        //             _ => panic!("Expression not identifier")
        //         }
        //     },
        //     s => panic!("Statement not expression. Got {:?}", s),
        // };
    }
}
