#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    //SPECIAL
    Illegal { location: usize, raw: String },

    EOF,

    // VALUES
    Ident { location: usize, raw: String },

    Int { location: usize, value: i64 },

    // OPERATORS
    Assign { location: usize },

    Plus { location: usize },

    Minus { location: usize },

    Multiply { location: usize },

    Divide { location: usize },

    Negation { location: usize },

    Equals { location: usize },

    NotEquals { location: usize },

    LessThan { location: usize },

    GreaterThan { location: usize },

    // SYNTAX
    Comma { location: usize },

    Semicolon { location: usize },

    LeftParen { location: usize },

    RightParen { location: usize },

    LeftBrace { location: usize },

    RightBrace { location: usize },

    // KEYWORDS
    Function { location: usize },

    Let { location: usize },

    True { location: usize },

    False { location: usize },

    If { location: usize },

    Else { location: usize },

    Return { location: usize },
}

pub fn lookup_ident(ident: &Token) -> Option<Token> {
    match ident {
        Token::Ident { location, raw } => match raw.as_str() {
            "fn" => Some(Token::Function {
                location: *location,
            }),
            "let" => Some(Token::Let {
                location: *location,
            }),
            "true" => Some(Token::True {
                location: *location,
            }),
            "false" => Some(Token::False {
                location: *location,
            }),
            "if" => Some(Token::If {
                location: *location,
            }),
            "else" => Some(Token::Else {
                location: *location,
            }),
            "return" => Some(Token::Return {
                location: *location,
            }),
            _ => None,
        },
        _ => None,
    }
}
