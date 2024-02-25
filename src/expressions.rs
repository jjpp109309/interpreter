use crate::ast::Node;

#[derive(Debug)]
pub enum Expression {
    Prefix,
    Infix,
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        todo!()
    }

    fn string(&self) -> String {
        todo!()
    }
}
