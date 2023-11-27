use crate::tokens::Token;

pub trait Node {
    fn token_literal(&self) -> String;
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
}

pub struct Expression {
    pub token: Token
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
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
}

pub struct Identifier {
    pub token: Token,
    pub value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
}

impl ExpressionNode for Identifier {
    fn expression_node(&self) {}
}
