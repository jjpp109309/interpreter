use crate::tokens::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait StatementNode {
    fn statement_node(&self);
}

trait ExpressionNode {
    fn expression_node(&self);
}

struct Statement {
    token: Token
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
}

struct Expression {
    token: Token
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
}

pub struct Program {
    statements: Vec<Statement>,
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

struct Identifier {
    token: Token,
    value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
}

impl ExpressionNode for Identifier {
    fn expression_node(&self) {}
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
}

impl StatementNode for LetStatement {
    fn statement_node(&self) {}
}