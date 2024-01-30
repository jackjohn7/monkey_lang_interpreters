use lexer::tokens::Token;
use lexer::Lexer;
use std::io::{stdin, stdout, Write};

fn main() {
    println!("Welcome to the Monkey-Lang REPL");
    loop {
        let mut input = String::new();
        print!(">> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Input error!");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        let mut tok = Lexer::new(&input);
        loop {
            match tok.next_token() {
                Token::EOF => {
                    break;
                }
                a => {
                    println!("{:?}", a);
                }
            }
        }
    }
}
