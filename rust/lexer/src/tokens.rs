use std::str;

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

struct Tokenizer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        let mut result = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        result.read_char();
        result
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let mut skip_read = false;
        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    let result = Token::Equals {
                        location: self.position,
                    };
                    self.read_char();
                    result
                } else {
                    Token::Assign {
                        location: self.position,
                    }
                }
            }
            b';' => Token::Semicolon {
                location: self.position,
            },
            b'(' => Token::LeftParen {
                location: self.position,
            },
            b')' => Token::RightParen {
                location: self.position,
            },
            b',' => Token::Comma {
                location: self.position,
            },
            b'+' => Token::Plus {
                location: self.position,
            },
            b'-' => Token::Minus {
                location: self.position,
            },
            b'*' => Token::Multiply {
                location: self.position,
            },
            b'/' => Token::Divide {
                location: self.position,
            },
            b'!' => {
                if self.peek_char() == b'=' {
                    let result = Token::NotEquals {
                        location: self.position,
                    };
                    self.read_char();
                    result
                } else {
                    Token::Negation {
                        location: self.position,
                    }
                }
            }
            b'<' => Token::LessThan {
                location: self.position,
            },
            b'>' => Token::GreaterThan {
                location: self.position,
            },
            b'{' => Token::LeftBrace {
                location: self.position,
            },
            b'}' => Token::RightBrace {
                location: self.position,
            },
            0 => Token::EOF,
            a => {
                if a.is_ascii_alphabetic() || a == b'_' {
                    skip_read = true;
                    let ident = self.read_identifier();
                    match lookup_ident(&ident) {
                        Some(t) => t,
                        None => ident,
                    }
                } else if a.is_ascii_digit() {
                    skip_read = true;
                    self.read_number()
                } else {
                    Token::Illegal {
                        location: self.position,
                        raw: a.to_string(),
                    }
                }
            }
        };
        if !skip_read {
            self.read_char();
        }

        token
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }
        Token::Ident {
            location: position,
            raw: str::from_utf8(&self.input.as_bytes()[position..self.position])
                .unwrap()
                .to_owned(),
        }
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        Token::Int {
            location: position,
            value: str::from_utf8(&self.input.as_bytes()[position..self.position])
                .unwrap()
                .parse::<i64>()
                .unwrap(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}

fn lookup_ident(ident: &Token) -> Option<Token> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = String::from("=+(){},;");
        let tests: Vec<Token> = vec![
            Token::Assign { location: 0 },
            Token::Plus { location: 1 },
            Token::LeftParen { location: 2 },
            Token::RightParen { location: 3 },
            Token::LeftBrace { location: 4 },
            Token::RightBrace { location: 5 },
            Token::Comma { location: 6 },
            Token::Semicolon { location: 7 },
        ];

        let mut tokenizer = Tokenizer::new(input);

        for (i, tt) in tests.iter().enumerate() {
            let tok = tokenizer.next_token();
            assert_eq!(&tok, tt, "test {} failed", i);
        }
    }

    #[test]
    fn test_tokenizing_file() {
        let input = include_str!("../../example.monke");
        let tests: Vec<Token> = vec![
            Token::Let { location: 0 },
            Token::Ident {
                location: 4,
                raw: String::from("five"),
            },
            Token::Assign { location: 9 },
            Token::Int {
                location: 11,
                value: 5,
            },
            Token::Semicolon { location: 12 },
            Token::Let { location: 14 },
            Token::Ident {
                location: 18,
                raw: String::from("ten"),
            },
            Token::Assign { location: 22 },
            Token::Int {
                location: 24,
                value: 10,
            },
            Token::Semicolon { location: 26 },
            Token::Let { location: 29 },
            Token::Ident {
                location: 33,
                raw: String::from("add"),
            },
            Token::Assign { location: 37 },
            Token::Function { location: 39 },
            Token::LeftParen { location: 41 },
            Token::Ident {
                location: 42,
                raw: String::from("x"),
            },
            Token::Comma { location: 43 },
            Token::Ident {
                location: 45,
                raw: String::from("y"),
            },
            Token::RightParen { location: 46 },
            Token::LeftBrace { location: 48 },
            Token::Ident {
                location: 52,
                raw: String::from("x"),
            },
            Token::Plus { location: 54 },
            Token::Ident {
                location: 56,
                raw: String::from("y"),
            },
            Token::Semicolon { location: 57 },
            Token::RightBrace { location: 59 },
            Token::Semicolon { location: 60 },
            Token::Let { location: 63 },
            Token::Ident {
                location: 67,
                raw: String::from("result"),
            },
            Token::Assign { location: 74 },
            Token::Ident {
                location: 76,
                raw: String::from("add"),
            },
            Token::LeftParen { location: 79 },
            Token::Ident {
                location: 80,
                raw: String::from("five"),
            },
            Token::Comma { location: 84 },
            Token::Ident {
                location: 86,
                raw: String::from("ten"),
            },
            Token::RightParen { location: 89 },
            Token::Semicolon { location: 90 },
            Token::Negation { location: 92 },
            Token::Minus { location: 93 },
            Token::Divide { location: 94 },
            Token::Multiply { location: 95 },
            Token::Int {
                location: 96,
                value: 5,
            },
            Token::Semicolon { location: 97 },
            Token::Int {
                location: 99,
                value: 5,
            },
            Token::LessThan { location: 101 },
            Token::Int {
                location: 103,
                value: 10,
            },
            Token::GreaterThan { location: 106 },
            Token::Int {
                location: 108,
                value: 5,
            },
            Token::Semicolon { location: 109 },
            Token::If { location: 112 },
            Token::LeftParen { location: 115 },
            Token::Int {
                location: 116,
                value: 5,
            },
            Token::LessThan { location: 118 },
            Token::Int {
                location: 120,
                value: 10,
            },
            Token::RightParen { location: 122 },
            Token::LeftBrace { location: 124 },
            Token::Return { location: 128 },
            Token::True { location: 135 },
            Token::Semicolon { location: 139 },
            Token::RightBrace { location: 141 },
            Token::Else { location: 143 },
            Token::LeftBrace { location: 148 },
            Token::Return { location: 152 },
            Token::False { location: 159 },
            Token::Semicolon { location: 164 },
            Token::RightBrace { location: 166 },
            Token::Int {
                location: 169,
                value: 10,
            },
            Token::Equals { location: 172 },
            Token::Int {
                location: 175,
                value: 10,
            },
            Token::Semicolon { location: 177 },
            Token::Int {
                location: 179,
                value: 10,
            },
            Token::NotEquals { location: 182 },
            Token::Int {
                location: 185,
                value: 9,
            },
            Token::Semicolon { location: 186 },
            Token::EOF,
        ];
        let mut tokenizer = Tokenizer::new(input.to_string());

        for (i, tt) in tests.iter().enumerate() {
            let tok = tokenizer.next_token();
            println!("{:?}", tok);
            assert_eq!(&tok, tt, "test {} failed", i);
        }
    }

    #[test]
    fn test_all_operators() {
        let input = "+-*/!<>";

        let tests: Vec<Token> = vec![
            Token::Plus { location: 0 },
            Token::Minus { location: 1 },
            Token::Multiply { location: 2 },
            Token::Divide { location: 3 },
            Token::Negation { location: 4 },
            Token::LessThan { location: 5 },
            Token::GreaterThan { location: 6 },
        ];

        let mut tokenizer = Tokenizer::new(input.to_string());

        for (i, tt) in tests.iter().enumerate() {
            let tok = tokenizer.next_token();
            assert_eq!(&tok, tt, "test {} failed", i);
        }
    }

    #[test]
    fn test_new_keywords() {
        let input = "if (5 < 10) {
return true;
} else {
return false;
}";

        let tests: Vec<Token> = vec![
            Token::If { location: 0 },
            Token::LeftParen { location: 3 },
            Token::Int {
                location: 4,
                value: 5,
            },
            Token::LessThan { location: 6 },
            Token::Int {
                location: 8,
                value: 10,
            },
            Token::RightParen { location: 10 },
            Token::LeftBrace { location: 12 },
            Token::Return { location: 14 },
            Token::True { location: 21 },
            Token::Semicolon { location: 25 },
            Token::RightBrace { location: 27 },
            Token::Else { location: 29 },
            Token::LeftBrace { location: 34 },
            Token::Return { location: 36 },
            Token::False { location: 43 },
            Token::Semicolon { location: 48 },
            Token::RightBrace { location: 50 },
        ];

        let mut tokenizer = Tokenizer::new(input.to_string());

        for (i, tt) in tests.iter().enumerate() {
            let tok = tokenizer.next_token();
            assert_eq!(&tok, tt, "test {} failed", i);
        }
    }

    #[test]
    fn test_identifiers() {
        let input = "add10 apple_bottom jeans_3_boots _ignored";
        let tests: Vec<Token> = vec![
            Token::Ident {
                location: 0,
                raw: String::from("add10"),
            },
            Token::Ident {
                location: 6,
                raw: String::from("apple_bottom"),
            },
            Token::Ident {
                location: 19,
                raw: String::from("jeans_3_boots"),
            },
            Token::Ident {
                location: 33,
                raw: String::from("_ignored"),
            },
        ];
        let mut tokenizer = Tokenizer::new(input.to_string());

        for (i, tt) in tests.iter().enumerate() {
            let tok = tokenizer.next_token();
            assert_eq!(&tok, tt, "test {} failed", i);
        }
    }
}
