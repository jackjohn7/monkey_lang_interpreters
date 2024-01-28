use lexer::tokens::Token;

pub trait Node {
    fn literal(&self) -> String;
}

pub trait Statement {
    fn statement_node(&self) -> ();
}

pub trait Expression {
    fn expression_node(&self) -> ();
}
