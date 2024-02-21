use crate::tokens;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug)]
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
        match self {
            Statement::Let(stmt) => stmt.string(),
            Statement::Return(stmt) => stmt.string(),
            Statement::Expression(stmt) => stmt.string(),
        }
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

#[derive(Debug)]
pub struct LetStatement {
    pub token: tokens::Token,
    pub name: Identifier,
    // pub value: Identifier,
}

impl LetStatement {
    fn string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str(&self.token.literal);
        buffer.push_str(" ");
        buffer.push_str(&self.name.token.literal);
        buffer.push_str(" = ");
        // TODO: expression value string
        buffer.push_str(";");

        buffer
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: tokens::Token,
    pub value: String,
}

impl Identifier {
    fn string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: tokens::Token,
    // pub value: Expression,
}

impl ReturnStatement {
    fn string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str(&self.token.literal);
        buffer.push_str(" ");
        // TODO: expression value string
        buffer.push_str(";");

        buffer
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: tokens::Token,
    pub expression: Expression,
}

impl ExpressionStatement {
    fn string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str(&self.expression.string());

        buffer
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier)
}

impl Expression {
    fn string(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn string() {
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement {
                    token: tokens::Token {
                        ttype: tokens::TokenType::Let,
                        literal: String::from("let"),
                    },
                    name: Identifier {
                        token: tokens::Token {
                            ttype: tokens::TokenType::Ident,
                            literal: String::from("myVar")
                        },
                        value: String::from("myVar")
                    }
                })
            ]
        };

        let expected = String::from("let myVar = ;");
        let string = program.string();

        assert_eq!(expected, string, "Wrong program. expected {} got {}", string, expected)
    }
}
