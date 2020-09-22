use std::cell::Cell;

use crate::token::{Token, TokenType};
use crate::parser::OpType::{Identifier, Plus};

#[derive(PartialOrd, PartialEq, Debug)]
pub enum OpType {
    Asterisk,
    Identifier,
    Minus,
    Percent,
    Plus,
    Root,
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
    op_type: OpType,
    token: Token,
    args: Vec<Node>,
}

pub fn sum(token_stack: &mut TokenStack) -> Node {
    let mut sum = number(token_stack);

    while token_stack.tokens.get_mut().len() > 0 {
        let op: Token = token_stack.tokens.get_mut().pop().unwrap();
        if op.t_type != TokenType::Plus {
            token_stack.tokens.get_mut().push(op);
            break;
        }
        sum = Node { op_type: Plus, token: op, args: vec![sum, number(token_stack)] };
    }
    sum
}

pub fn number(token_stack: &mut TokenStack) -> Node {
    Node { op_type: Identifier, token: token_stack.tokens.get_mut().pop().unwrap(), args: vec![] }
}