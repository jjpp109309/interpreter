use crate::lexer::Lexer;
use crate::ast::Program;
use crate::tokens::Token;

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

    fn parse_program() -> Program {
        todo!()
    }
}
