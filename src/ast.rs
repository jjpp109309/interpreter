use crate::tokens;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(stmt) => stmt.token.literal.to_owned(),
            Statement::Return(stmt) => stmt.token.literal.to_owned(),
            Statement::Expression(stmt) => stmt.token.literal.to_owned(),
        }
    }

    fn string(&self) -> String {
        String::from("")
    }
}

impl Statement {
    pub fn name_token_literal(&self) -> String {
        match self {
            Statement::Let(stmt) => stmt.name.token.literal.to_owned(),
            Statement::Return(_) => panic!("Return statement does not have name field"),
            Statement::Expression(_) => panic!("Return statement does not have name field")
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }

    fn string(&self) -> String {
        self
            .statements
            .iter()
            .map(|s| s.string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub struct LetStatement {
    pub token: tokens::Token,
    pub name: Identifier,
    // pub value: Identifier,
}

pub struct Identifier {
    pub token: tokens::Token,
    pub value: String,
}

pub struct ReturnStatement {
    pub token: tokens::Token,
    // pub value: Expression,
}

pub struct ExpressionStatement {
    pub token: tokens::Token,
    pub expression: Expression,
}

pub struct Expression {

}
