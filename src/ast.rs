use crate::tokens::Token;
use crate::expressions::Expression;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let { token: Token, identifier: Token },
    Return {token: Token },
    Expression { token: Token, expression: Expression },
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { token, .. } => token.string(),
            Statement::Return { token } => token.string(),
            Statement::Expression { token, .. } => token.string(),
        }
    }

    fn string(&self) -> String {
        let mut buffer = String::new();

        match self {
            Statement::Let { token, identifier } => {

                buffer.push_str(&token.string());
                buffer.push_str(" ");
                buffer.push_str(&identifier.string());
                buffer.push_str(" = ");
                // TODO: expression value string
                buffer.push_str(";");
            },
            Statement::Return { token } => {
                buffer.push_str(&token.string());
                buffer.push_str(" ");
                // TODO: expression value string
                buffer.push_str(";");
            },
            Statement::Expression { expression, .. } => {
                buffer.push_str(&expression.string());
            }
        }

        buffer
    }
}

impl Statement {
    pub fn name_token_literal(&self) -> String {
        match self {
            Statement::Let { identifier, .. } => identifier.string(),
            Statement::Return { .. } => panic!("Return statement does not have identifier"),
            Statement::Expression { .. } => panic!("Return statement does not have identifier")
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

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn string() {
        let program = Program {
            statements: vec![
                Statement::Let {
                    token: Token::Let,
                    identifier: Token::Ident("myVar".to_string()),
                }
            ]
        };

        let expected = String::from("let myVar = ;");
        let string = program.string();

        assert_eq!(expected, string, "Wrong program. expected {} got {}", string, expected)
    }
}
