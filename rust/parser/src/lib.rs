use lexer::{tokens::Token, Lexer};

pub mod ast;
use ast::{Let, Program, Return, Statement};

#[derive(Clone, Debug)]
pub struct ParserError(String);

pub struct Parser<'a> {
    lex: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lex: Lexer<'a>) -> Self {
        let current_token = lex.next_token();
        let peek_token = lex.next_token();
        Parser {
            lex,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };

        loop {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            }
            self.next_token();

            if self.peek_token == Token::EOF {
                break;
            }
        }

        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current_token {
            tok @ Token::Let { .. } => match self.parse_let_statement(tok.clone()) {
                Ok(l) => Some(Statement::Let(l)),
                Err(msg) => {
                    self.errors.push(msg.to_owned());
                    None
                }
            },
            tok @ Token::Return { .. } => match self.parse_return_statement(tok.clone()) {
                Ok(r) => Some(Statement::Return(r)),
                Err(msg) => {
                    self.errors.push(msg.to_owned());
                    None
                }
            },
            _ => None,
        }
    }

    pub fn parse_return_statement(&mut self, ret_tok: Token) -> Result<Return, ParserError> {
        self.next_token();
        let temp_current = self.current_token.clone();
        loop {
            match &self.current_token {
                Token::Semicolon { .. } => {
                    break Ok(Return {
                        token: ret_tok,
                        return_value: ast::Expression::Identifier(temp_current),
                    })
                }
                _ => {
                    self.next_token();
                }
            }
        }
    }

    pub fn parse_let_statement(&mut self, let_tok: Token) -> Result<Let, ParserError> {
        match &self.peek_token {
            Token::Ident { .. } => {
                self.next_token();
                let ident_tok = self.current_token.clone();
                match &self.peek_token {
                    Token::Assign { .. } => {
                        self.next_token();
                        let result = Ok(Let {
                            token: let_tok,
                            identifier: ast::Expression::Identifier(ident_tok.clone()),
                            value: ast::Expression::Identifier(self.peek_token.clone()), // TEMPORARY
                        });

                        loop {
                            match &self.current_token {
                                Token::Semicolon { .. } => break result,
                                _ => {
                                    self.next_token();
                                }
                            }
                        }
                    }
                    a => Err(ParserError(format!(
                        "Unexpected Token '{:?}'. Expected assignment operator",
                        a
                    ))),
                }
            }
            a => Err(ParserError(format!(
                "Unexpected Token '{:?}'. Expected identifier",
                a
            ))),
        }
    }

    pub fn errors(&self) -> Vec<ParserError> {
        self.errors.clone()
    }
}

#[cfg(test)]
mod tests {
    use lexer::{tokens::Token, Lexer};

    use crate::{
        ast::{Expression, Return, Statement},
        Parser,
    };

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(program.statements.len(), 3);

        let ident_names = vec!["x", "y", "foobar"];

        for (i, t) in ident_names.iter().enumerate() {
            let stmt = &program.statements[i];

            if let Statement::Let(let_stmt) = stmt {
                let Expression::Identifier(ident) = &let_stmt.identifier;
                if let Token::Ident { raw, .. } = ident {
                    assert_eq!(raw, t.to_owned());
                }
            } else {
                // temporary
                panic!("Statement is not a let statement");
            }
        }
    }

    #[test]
    fn test_failing_let_statements() {
        let input = "let x 5;
let = 10;
let 838383;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let _ = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors().len(), 3);
    }

    #[test]
    fn test_return_statements() {
        let input = "return 5;
return 10;
return 993322;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(program.statements.len(), 3);

        assert_eq!(parser.errors().len(), 0);

        for (i, stmt) in program.statements.iter().enumerate() {
            if let Statement::Return(ret) = stmt {
            } else {
                panic!("Statement should be a return statement");
            }
        }
    }
}
