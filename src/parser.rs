use crate::tokens::Token;
use crate::lexer::Lexer;
use crate::ast;
use std::collections::HashMap;

// type PrefixFn = fn() -> ast::Expression;
// type InfixFn = fn(ast::Expression) -> ast::Expression;

// enum ParseFn {
//     Infix(InfixFn),
//     Prefix(PrefixFn),
// }

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    // prefix_parse_fns: HashMap<String, PrefixFn>,
    // infix_parse_fns: HashMap<String, InfixFn>,
}

impl Parser {
    fn new(mut l: Lexer) -> Parser {
        let cur_token = l.next_token();
        let peek_token = l.next_token();

        // let prefix_parse_fns = HashMap::new();
        // let infix_parse_fns = HashMap::new();

        Parser { lexer: l, cur_token, peek_token }
        // Parser { lexer: l, cur_token, peek_token, prefix_parse_fns, infix_parse_fns }
    } 

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(mut self) -> ast::Program {
        let mut statements: Vec<ast::Statement> = vec![];

        while self.cur_token != Token::Eof {
            let statement = self.parse_statement();

            statements.push(statement);
            self.next_token();
        }

        ast::Program { statements }
    }

    fn parse_statement(&mut self) -> ast::Statement {
        match self.cur_token {
            Token::Let(_) => self.parse_let_statement(),
            Token::Return(_) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> ast::Statement {
        let token = self.cur_token.clone();

        match self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => panic!("Next token not Ident")
        }

        let identifier = self.cur_token.clone();

        match self.peek_token {
            Token::Assign(_) => self.next_token(),
            _ => panic!("Next token not Ident")
        }

        // TODO: parse expression

        loop {
            match self.cur_token {
                Token::SemiColon(_) => break,
                _ => self.next_token(),
            }
        }

        ast::Statement::Let { token, identifier }
    }

    fn parse_return_statement(&mut self) -> ast::Statement {
        let token = self.cur_token.clone();

        self.next_token();

        // TODO: parse expression

        loop {
            match self.cur_token {
                Token::SemiColon(_) => break,
                _ => self.next_token(),
            }
        }

        ast::Statement::Return { token }
    }

    fn parse_expression_statement(&mut self) -> ast::Statement {todo!()}

    //
    // fn register_parse_fn(&mut self, token_type: &String, func: ParseFn) {
    //     match func {
    //         ParseFn::Infix(f) => {
    //             self.infix_parse_fns.insert(token_type.to_string(), f);
    //         },
    //         ParseFn::Prefix(f) => {
    //             self.prefix_parse_fns.insert(token_type.to_string(), f);
    //         },
    //     };
    // }
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

//     #[test]
//     fn identifier_statement() {
//         let input = String::from("\
// foobar;");
//
//         let l = Lexer::new(&input);
//         let p = Parser::new(l);
//         let program = p.parse_program();
//
//         let n = program.statements.len();
//         assert_eq!(n, 1, "Program has not enough statements. Got {}", n);
//
//         let statement = &program.statements[0];
//
//         match statement {
//             ast::Statement::Expression(s) => {
//                 match &s.expression {
//                     ast::Expression::Identifier(i) => {
//                         if i.value != String::from("foobar") {
//                             panic!("Identifier value not correct. Got {:?}", i)
//                         }
//                     },
//                     _ => panic!("Expression not identifier")
//                 }
//             },
//             s => panic!("Statement not expression. Got {:?}", s),
//         };
//     }
}
