trait Node {
    fn token_literal(&self) -> String;
}

struct Statement;

impl Node for Statement {
    fn token_literal(&self) -> String {
        todo!()
    }
}

struct Program {
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
