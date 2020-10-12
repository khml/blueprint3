use crate::parser::{Node, OpType};
use std::borrow::Borrow;

pub fn evaluate(node: &Node) -> f64 {
    if node.args.len() == 0 {
        return node.token.value.parse().unwrap();
    }
    assert_eq!(node.args.len(), 2);
    let left = evaluate(node.args[0].borrow());
    let right = evaluate(node.args[1].borrow());
    match node.op_type {
        OpType::Asterisk => left * right,
        OpType::Minus => left - right,
        OpType::Percent => left % right,
        OpType::Plus => left + right,
        OpType::Slash => left / right,
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::TokenStack;
    use crate::token::tokenize;
    use crate::parser::parse;

    use super::evaluate;

    #[test]
    fn test_evaluate() {
        {
            let node = parse(&mut TokenStack::new(tokenize("(12+2)*3").unwrap()));
            let expected: f64 = 42.0;
            assert_eq!(evaluate(&node), expected);
        }
        {
            let node = parse(&mut TokenStack::new(tokenize("(12/2) - 3").unwrap()));
            let expected: f64 = 3.0;
            assert_eq!(evaluate(&node), expected);
        }
        {
            let node = parse(&mut TokenStack::new(tokenize("(12%20) - 10").unwrap()));
            let expected: f64 = 2.0;
            assert_eq!(evaluate(&node), expected);
        }
        {
            let node = parse(&mut TokenStack::new(tokenize("0.8 + 3").unwrap()));
            let expected: f64 = 3.8;
            assert_eq!(evaluate(&node), expected);
        }
    }
}