use crate::lexer::{Lexer, Token};

pub struct Parser {
    postfix: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            postfix: vec![],
        }
    }

    pub fn add_to_postfix(&mut self, token: Token) {
        self.postfix.push(token);
    }

}

pub fn calculate(operator: Token, op1: i32, op2: i32) -> i32 {
    match operator {
        Token::PLUS() => op1 + op2,
        Token::MINUS() => op1 - op2,
        Token::MULTIPLY() => op1 * op2,
        Token::DIVISION() => op1 / op2,
        _ => -1
    }
}

pub fn form_postfix(mut lexer: Lexer) {
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

        for token in &parser.postfix {
            print!("{}, ", token);
        }
        println!();
    } else {
        println!("Lexical analysis has failed!");
    }
    if op_stack.contains(&Token::LPAREN()) || op_stack.contains(&Token::RPAREN()) {
        println!("Parenthesis not matched!");
    } else {
        println!("Expression value is: {}", evaluate_postfix(parser));
    }
}

pub fn evaluate_postfix(parser: Parser) -> i32 {
    let mut op_stack: Vec<Token> = vec![];
    for token in parser.postfix {
        if token.is_operand() {
            op_stack.push(token);
        } else {
            let op1 = op_stack.pop().unwrap().get_value();
            let op2 = op_stack.pop().unwrap().get_value();
            op_stack.push(Token::NUMBER(calculate(token, op2, op1)));
        }
    }
    op_stack.pop().unwrap().get_value()
}

