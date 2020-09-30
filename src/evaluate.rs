use crate::parser::{Node, OpType};
use std::borrow::Borrow;

pub fn evaluate(node: &Node) -> u32 {
    if node.args.len() == 0 {
        return node.token.value.parse().unwrap();
    }
    assert_eq!(node.args.len(), 2);
    let left = evaluate(node.args[0].borrow());
    let right = evaluate(node.args[1].borrow());
    match node.op_type  {
        OpType::Asterisk => left * right,
        OpType::Minus => left - right,
        OpType::Percent => left % right,
        OpType::Plus => left + right,
        OpType::Slash => left / right,
        _ => unimplemented!(),
    }
}
