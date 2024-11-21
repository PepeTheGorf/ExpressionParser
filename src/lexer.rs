use std::fmt;
use std::fmt::{Formatter};
use std::io::{Write};


pub enum Token {
    PLUS(),
    MINUS(),
    MULTIPLY(),
    DIVISION(),
    RPAREN(),
    LPAREN(),
    NUMBER(i32),
}

impl Token {
    pub fn is_operand(&self) -> bool {
        match self {
            Token::NUMBER(_value) => true,
            _ => false
        }
    }

    pub fn get_value(&self) -> i32 {
        match self {
            Token::NUMBER(value) => return *value,
            _ => -1
        }
    }

    pub fn get_precedence(&self) -> i8 {
        match self {
            Token::MULTIPLY() | Token::DIVISION() => 2,
            Token::PLUS() | Token::MINUS() => 1,
            _ => -1
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        if other.to_string().eq(&self.to_string()) {
            return true;
        }
        false
    }
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
    pub tokens: Vec<Token>
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