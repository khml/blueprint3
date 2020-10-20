use std::borrow::Borrow;

use crate::parser::{Node, OpType};
use crate::context::Context;

pub fn evaluate(node: &Node, context: &mut Context) -> f64 {
    match node.op_type {
        OpType::Assignment => {
            let assign_value = evaluate(&node.args[0], context);
            context.var_tbl.get_mut().insert(node.token.value.clone(), assign_value);
            return 0.0;
        }
        OpType::Identifier => {
            return context.var_tbl.get_mut().get(&node.token.value).unwrap().clone();
        }
        OpType::Number => {
            return node.token.value.parse().unwrap();
        }
        _ => {
            assert_eq!(node.args.len(), 2, "args error. node = {:?}", node);
            let left = evaluate(node.args[0].borrow(), context);
            let right = evaluate(node.args[1].borrow(), context);
            match node.op_type {
                OpType::Asterisk => left * right,
                OpType::Minus => left - right,
                OpType::Percent => left % right,
                OpType::Plus => left + right,
                OpType::Slash => left / right,
                _ => unimplemented!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::TokenStack;
    use crate::token::tokenize;
    use crate::parser::parse;

    use super::evaluate;
    use crate::context::Context;

    #[test]
    fn test_evaluate() {
        {
            let mut context = Context::new();
            let node = parse(&mut TokenStack::new(tokenize("(12+2)*3").unwrap()));
            let expected: f64 = 42.0;
            assert_eq!(evaluate(&node, &mut context), expected);
        }
        {
            let mut context = Context::new();
            let node = parse(&mut TokenStack::new(tokenize("(12/2) - 3").unwrap()));
            let expected: f64 = 3.0;
            assert_eq!(evaluate(&node, &mut context), expected);
        }
        {
            let mut context = Context::new();
            let node = parse(&mut TokenStack::new(tokenize("(12%20) - 10").unwrap()));
            let expected: f64 = 2.0;
            assert_eq!(evaluate(&node, &mut context), expected);
        }
        {
            let mut context = Context::new();
            let node = parse(&mut TokenStack::new(tokenize("0.8 + 3").unwrap()));
            let expected: f64 = 3.8;
            assert_eq!(evaluate(&node, &mut context), expected);
        }
    }
}