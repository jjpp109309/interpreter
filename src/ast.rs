use crate::tokens::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let { token: Token, identifier: Token },
    Return {token: Token },
    // Expression(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { token, .. } => token.string(),
            Statement::Return { token } => token.string(),
            // Statement::Expression(stmt) => stmt.token.literal.to_owned(),
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
            }
        }

        buffer
    }
}

impl Statement {
    pub fn name_token_literal(&self) -> String {
        match self {
            Statement::Let { token, .. } => token.string(),
            Statement::Return { .. } => panic!("Return statement does not have identifier"),
            // Statement::Expression(_) => panic!("Return statement does not have name field")
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

// #[derive(Debug)]
// pub struct ExpressionStatement {
//     pub token: tokens::Token,
//     pub expression: Expression,
// }
//
// impl ExpressionStatement {
//     fn string(&self) -> String {
//         let mut buffer = String::new();
//
//         buffer.push_str(&self.expression.string());
//
//         buffer
//     }
// }
//
// #[derive(Debug)]
// pub enum Expression {
//     Identifier(Identifier)
// }
//
// impl Expression {
//     fn string(&self) -> String {
//         todo!()
//     }
// }

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn string() {
        let program = Program {
            statements: vec![
                Statement::Let {
                    token: Token::Let("let".to_string()),
                    identifier: Token::Ident("myVar".to_string()),
                }
            ]
        };

        let expected = String::from("let myVar = ;");
        let string = program.string();

        assert_eq!(expected, string, "Wrong program. expected {} got {}", string, expected)
    }
}
