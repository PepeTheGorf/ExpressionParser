use crate::lexer;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    postfix: Vec<Token>,
    value: f32,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            postfix: vec![],
            value: 0.0,
        }
    }

    pub fn add_to_postfix(&mut self, token: Token) {
        self.postfix.push(token);
    }
}


//TODO: Convert infix to postfix notation then calculate the value.
pub fn form_postfix() {
    let input = String::from("((1 + 2) - 3 * (4 / 5)) + 6");
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new();

    let mut op_stack: Vec<Token> = vec![];

    if lexer.do_analysis() {
        for token in lexer.tokens {
            if token.is_operand() {
                parser.add_to_postfix(token);
            } else {
                if token.eq(&Token::LPAREN()) {
                    op_stack.push(token);
                } else if token.eq(&Token::RPAREN())  {
                    while !op_stack.last().unwrap().eq(&Token::LPAREN()) {
                        parser.postfix.push(op_stack.pop().unwrap());
                    }
                    op_stack.pop();
                } else {
                    if (op_stack.is_empty()
                        || token.get_precedence() > op_stack.last().unwrap().get_precedence())
                        || (op_stack.contains(&Token::LPAREN())) {
                        op_stack.push(token);
                    } else {
                        while !op_stack.is_empty() && op_stack.last().unwrap().get_precedence() >= token.get_precedence() {
                            parser.postfix.push(op_stack.pop().unwrap());
                        }
                        op_stack.push(token);
                    }
                }
            }
        }
        while !op_stack.is_empty() {
            parser.postfix.push(op_stack.pop().unwrap());
        }

        for token in parser.postfix {
            println!("{}", token);
        }
    } else {
        println!("Lexical analysis has failed!");
    }
}

