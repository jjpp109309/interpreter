use crate::tokens;

pub trait Node {
    fn token_literal(&self) -> String;
    fn name_token_literal(&self) -> String;
}

trait Statement {
    fn statement_node(&self);
}

trait Expression {
    fn expression_node(&self);
}

pub enum Statements {
    Let(Option<LetStatement>),
    Return(Option<ReturnStatement>),
    None,
}

pub struct Program {
    pub statements: Vec<Box<dyn Node>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }

    fn name_token_literal(&self) -> String {
        String::from("")
    }
}

pub struct LetStatement {
    pub token: tokens::Token,
    pub name: Identifier,
    // pub value: Identifier,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn name_token_literal(&self) -> String {
        self.name.token.literal.to_owned()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct Identifier {
    pub token: tokens::Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn name_token_literal(&self) -> String {
        panic!("Identifier does not have name attribute")
    }
}

impl Statement for Identifier {
    fn statement_node(&self) {}
}

pub struct ReturnStatement {
    pub token: tokens::Token,
    // pub value: Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn name_token_literal(&self) -> String {
        panic!("Return statement does not have name attribute")
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}
