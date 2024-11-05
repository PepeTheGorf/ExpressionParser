use std::error::Error;
use std::fmt;
use std::fmt::{Formatter, write};
use std::io::{self, Write};


enum Token {
    PLUS(),
    MINUS(),
    MULTIPLY(),
    DIVISION(),
    RPAREN(),
    LPAREN(),
    NUMBER(i32),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::PLUS() => write!(f, "PLUS"),
            Token::MINUS() => write!(f, "MINUS"),
            Token::MULTIPLY() => write!(f, "MULTIPLY"),
            Token::DIVISION() => write!(f, "DIVISION"),
            Token::RPAREN() => write!(f, "RPAREN"),
            Token::LPAREN() => write!(f, "LPAREN"),
            Token::NUMBER(value) => write!(f, "NUMBER({value})"),
        }
    }
}

pub struct Lexer {
    expression: String,
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new(expression: String) -> Self {
        Self {
            expression,
            tokens: vec![]
        }
    }

    pub fn do_analysis(&mut self) -> bool {
        let mut number_to_add = 0;
        let mut is_building_number = false;
        for ch in self.expression.chars() {
            match ch {
                '+' => {
                    if is_building_number {
                        self.tokens.push(Token::NUMBER(number_to_add));
                        number_to_add = 0;
                        is_building_number = false;
                    }
                    self.tokens.push(Token::PLUS());
                },
                '-' => {
                    if is_building_number {
                        self.tokens.push(Token::NUMBER(number_to_add));
                        number_to_add = 0;
                        is_building_number = false;
                    }
                    self.tokens.push(Token::MINUS());
                },
                '*' => {
                    if is_building_number {
                        self.tokens.push(Token::NUMBER(number_to_add));
                        number_to_add = 0;
                        is_building_number = false;
                    }
                    self.tokens.push(Token::MULTIPLY());
                },
                '/' => {
                    if is_building_number {
                        self.tokens.push(Token::NUMBER(number_to_add));
                        number_to_add = 0;
                        is_building_number = false;
                    }
                    self.tokens.push(Token::DIVISION());
                },
                '(' => {
                    if is_building_number {
                        self.tokens.push(Token::NUMBER(number_to_add));
                        number_to_add = 0;
                        is_building_number = false;
                    }
                    self.tokens.push(Token::LPAREN());
                },
                ')' => {
                    if is_building_number {
                        self.tokens.push(Token::NUMBER(number_to_add));
                        number_to_add = 0;
                        is_building_number = false;
                    }
                    self.tokens.push(Token::RPAREN());
                },
                '0'..='9' => {
                    let digit = (ch as i32) - ('0' as i32);
                    number_to_add = number_to_add * 10 + digit;
                    is_building_number = true;
                },
                ' ' => continue,
                _ => {
                    println!("Error! Invalid character '{}'", ch);
                    return false;
                },
            }
        }
        if is_building_number {
            self.tokens.push(Token::NUMBER(number_to_add));
        }
        true
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            if let Token::NUMBER(value) = token {
                println!("{}", value);
            } else {
                println!("{}", token);
            }
        }
    }


}

pub fn do_lex() {
    print!("Enter mathematical expression: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().to_string();

    let mut lexer = Lexer::new(input);

    if lexer.do_analysis() {
        lexer.print_tokens();
    } else {
        println!("Lexical analysis has failed!");
    }
}