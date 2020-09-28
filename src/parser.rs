use std::cell::Cell;

use crate::token::{Token, TokenType};

#[derive(PartialOrd, PartialEq, Debug)]
pub enum OpType {
    Asterisk,
    Minus,
    Number,
    Percent,
    Plus,
    Slash,
}

pub struct TokenStack {
    pub tokens: Cell<Vec<Token>>
}

impl TokenStack {
    pub fn new(mut _tokens: Vec<Token>) -> TokenStack {
        _tokens.reverse();
        TokenStack { tokens: Cell::new(_tokens) }
    }
}

#[derive(Debug)]
pub struct Node {
    pub op_type: OpType,
    pub token: Token,
    pub args: Vec<Node>,
}

pub fn sum(token_stack: &mut TokenStack) -> Node {
    let mut sum_term = mul(token_stack);

    while token_stack.tokens.get_mut().len() > 0 {
        let op: Token = token_stack.tokens.get_mut().pop().unwrap();
        let op_type;

        match op.t_type {
            TokenType::Plus => op_type = OpType::Plus,
            TokenType::Minus => op_type = OpType::Minus,
            _ => {
                token_stack.tokens.get_mut().push(op);
                break;
            }
        }
        sum_term = Node { op_type, token: op, args: vec![sum_term, mul(token_stack)] };
    }
    sum_term
}

pub fn mul(token_stack: &mut TokenStack) -> Node {
    let mut mul_term = number(token_stack);

    while token_stack.tokens.get_mut().len() > 0 {
        let op: Token = token_stack.tokens.get_mut().pop().unwrap();
        let op_type;

        match op.t_type {
            TokenType::Asterisk => op_type = OpType::Asterisk,
            TokenType::Slash => op_type = OpType::Slash,
            TokenType::Percent => op_type = OpType::Percent,
            _ => {
                token_stack.tokens.get_mut().push(op);
                break;
            }
        }
        mul_term = Node { op_type, token: op, args: vec![mul_term, number(token_stack)] };
    }
    mul_term
}

pub fn number(token_stack: &mut TokenStack) -> Node {
    Node { op_type: OpType::Number, token: token_stack.tokens.get_mut().pop().unwrap(), args: vec![] }
}