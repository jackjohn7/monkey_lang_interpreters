use lexer::tokens::Token;

#[derive(Debug)]
pub struct Let {
    pub token: Token,
    pub identifier: Expression,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Return {
    pub token: Token,
    pub return_value: Expression,
}

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Return(Return),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Token),
}

#[derive(Debug)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

// implementations
// pub trait Literable {
//     fn literal(&self) -> String;
// }
// impl Literable for Program {
//     fn literal(&self) -> String {
//         if self.statements.len() > 0 {
//             format!("{:?}", self.statements[0])
//         } else {
//             String::from("")
//         }
//     }
// }
// impl Literable for Token {
//     fn literal(&self) -> String {
//         match self {
//             Self::Illegal { raw, .. } => raw.clone(),
//             Self::EOF => String::from("END"),
//             Self::Ident { raw, .. } => raw.clone(),
//             Self::Int { value, .. } => format!("{}", value),
//             Self::Assign { .. } => String::from("="),
//             Self::Plus { .. } => String::from("+"),
//             Self::Minus { .. } => String::from("-"),
//             Self::Multiply { .. } => String::from("*"),
//             Self::Divide { .. } => String::from("/"),
//             Self::Negation { .. } => String::from("!"),
//             Self::Equals { .. } => String::from("=="),
//             Self::NotEquals { .. } => String::from("!="),
//             Self::LessThan { .. } => String::from("<"),
//             Self::GreaterThan { .. } => String::from(">"),
//             Self::Comma { .. } => String::from(","),
//             Self::Semicolon { .. } => String::from(";"),
//             Self::LeftParen { .. } => String::from("("),
//             Self::RightParen { .. } => String::from(")"),
//             Self::LeftBrace { .. } => String::from("{"),
//             Self::RightBrace { .. } => String::from("}"),
//             Self::Function { .. } => String::from("fn"),
//             Self::Let { .. } => String::from("let"),
//             Self::True { .. } => String::from("true"),
//             Self::False { .. } => String::from("false"),
//             Self::If { .. } => String::from("if"),
//             Self::Else { .. } => String::from("else"),
//             Self::Return { .. } => String::from("return"),
//         }
//     }
// }
