use crate::tokens::{Token, TokenType};

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait StatementNode {
    fn statement_node(&self);
}

pub trait ExpressionNode {
    fn expression_node(&self);
}

pub struct Statement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Expression>,
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        
        let mut result = match self.token.ttype {
            TokenType::Let => {
                let token = self.token_literal();
                let name = if let Some(x) = &self.name {
                    x.string() }
                else { "".to_string() };
                let equals = "=".to_string();
                let value = if let Some(x) = &self.value {
                    x.string() }
                else { "".to_string() };

                vec![token, name, equals, value].join(" ")
            },
            TokenType::Return => {
                let token = self.token_literal();
                let value = if let Some(x) = &self.value {
                    x.string()
                } else { "".to_string() };

                vec![token, value].join(" ")
            },
            _ => { String::from("") }
        };

        result.push_str(";");
        result
    }
}

pub struct Expression {
    pub token: Token
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        self.token_literal()
    }
}

pub struct Program {
    // pub statements: Vec<Statement>,
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
        self.statements.iter().map(|x| x.string()).collect()
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }

    fn string(&self) -> String {
        self.value.to_owned()
    }
}

impl ExpressionNode for Identifier {
    fn expression_node(&self) {}
}

mod test {
    use super::*;

    #[test]
    fn string() {
        let program = Program {
            statements: vec![
                Statement {
                    token: Token {
                        ttype: TokenType::Let,
                        literal: String::from("let"),
                    },
                    name: Some(Identifier {
                        token: Token {
                            ttype: TokenType::Ident,
                            literal: String::from("myVar")
                        },
                        value: String::from("myVar")
                    }),
                    value: Some(Expression {
                        token: Token {
                            ttype: TokenType::Ident,
                            literal: String::from("anotherVar")
                        },
                    })
                }
            ]
        };

        assert_eq!(program.string(), String::from("let myVar = anotherVar;"))
    }
}
