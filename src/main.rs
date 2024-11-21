use std::io;
use std::io::Write;
use crate::lexer::Lexer;

mod lexer;
mod parser;

fn main() {
    let mut input = String::new();

    loop {
        input = String::new();
        print!("Enter mathematical expression: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_string();
        if input == "quit" {
            break;
        }
        let lexer = Lexer::new(input);

        parser::form_postfix(lexer);
    }
}
