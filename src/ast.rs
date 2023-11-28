trait Node {
    fn token_literal(&self) -> String;
}

trait Statement {
    fn statement_node(&self);
}

trait Expression {
    fn statement_node(&self);
}

struct Program {
    statements: Vec<Box<dyn Node>>,
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


