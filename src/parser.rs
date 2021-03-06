use std::cell::Cell;

use crate::token::{Token, TokenType};

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub op_type: OpType,
    pub token: Token,
    pub args: Vec<Node>,
}

pub fn parse(token_stack: &mut TokenStack) -> Node {
    sum(token_stack)
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

pub fn number(token_stack: &mut TokenStack) -> Node {
    Node { op_type: OpType::Number, token: token_stack.tokens.get_mut().pop().unwrap(), args: vec![] }
}

pub fn priority(token_stack: &mut TokenStack) -> Result<Node, String> {
    if token_stack.tokens.get_mut().last().unwrap().t_type != TokenType::ParenthesisLeft {
        return Ok(number(token_stack));
    }
    token_stack.tokens.get_mut().pop().unwrap();
    let node = sum(token_stack);

    let t_type = token_stack.tokens.get_mut().pop().unwrap().t_type;
    match t_type {
        TokenType::ParenthesisRight => Ok(node),
        _ => Err(format!("Expected: {:?} token, but {:?} given", TokenType::ParenthesisRight, t_type)),
    }
}

pub fn mul(token_stack: &mut TokenStack) -> Node {
    let mut priority_term = priority(token_stack).unwrap();

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
        priority_term = Node { op_type, token: op, args: vec![priority_term, priority(token_stack).unwrap()] };
    }
    priority_term
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::Node;
    use super::OpType;
    use super::Token;
    use super::TokenStack;
    use super::TokenType;
    use super::number;
    use super::priority;
    use super::sum;
    use super::mul;

    #[test]
    fn test_number() {
        let mut token_stack = TokenStack {
            tokens: Cell::new(vec![Token { t_type: TokenType::Number, value: "1".to_string() }])
        };
        let expected = Node {
            op_type: OpType::Number,
            token: Token {
                t_type: TokenType::Number,
                value: "1".to_string(),
            },
            args: vec![],
        };
        assert_eq!(number(&mut token_stack), expected);
    }

    #[test]
    fn test_sum() {
        let mut token_stack = TokenStack {
            tokens: Cell::new(vec![
                Token { t_type: TokenType::Number, value: "2".to_string() },
                Token { t_type: TokenType::Plus, value: "+".to_string() },
                Token { t_type: TokenType::Number, value: "1".to_string() },
            ])
        };
        let one_plus_two = Node {
            op_type: OpType::Plus,
            token: Token {
                t_type: TokenType::Plus,
                value: "+".to_string(),
            },
            args: vec![
                Node {
                    op_type: OpType::Number,
                    token: Token {
                        t_type: TokenType::Number,
                        value: "1".to_string(),
                    },
                    args: vec![],
                },
                Node {
                    op_type: OpType::Number,
                    token: Token {
                        t_type: TokenType::Number,
                        value: "2".to_string(),
                    },
                    args: vec![],
                }
            ],
        };
        assert_eq!(sum(&mut token_stack), one_plus_two);


        let mut token_stack = TokenStack {
            tokens: Cell::new(vec![
                Token { t_type: TokenType::Number, value: "3".to_string() },
                Token { t_type: TokenType::Plus, value: "+".to_string() },
                Token { t_type: TokenType::Number, value: "2".to_string() },
                Token { t_type: TokenType::Plus, value: "+".to_string() },
                Token { t_type: TokenType::Number, value: "1".to_string() },
            ])
        };
        let expected = Node {
            op_type: OpType::Plus,
            token: Token {
                t_type: TokenType::Plus,
                value: "+".to_string(),
            },
            args: vec![
                one_plus_two,
                Node {
                    op_type: OpType::Number,
                    token: Token {
                        t_type: TokenType::Number,
                        value: "3".to_string(),
                    },
                    args: vec![],
                },
            ],
        };
        assert_eq!(sum(&mut token_stack), expected);
    }

    #[test]
    fn test_mul() {
        let mut token_stack = TokenStack {
            tokens: Cell::new(vec![
                Token { t_type: TokenType::Number, value: "3".to_string() },
                Token { t_type: TokenType::Asterisk, value: "*".to_string() },
                Token { t_type: TokenType::Number, value: "1".to_string() },
            ])
        };
        let one_product_three = Node {
            op_type: OpType::Asterisk,
            token: Token {
                t_type: TokenType::Asterisk,
                value: "*".to_string(),
            },
            args: vec![
                Node {
                    op_type: OpType::Number,
                    token: Token {
                        t_type: TokenType::Number,
                        value: "1".to_string(),
                    },
                    args: vec![],
                },
                Node {
                    op_type: OpType::Number,
                    token: Token {
                        t_type: TokenType::Number,
                        value: "3".to_string(),
                    },
                    args: vec![],
                }
            ],
        };
        assert_eq!(mul(&mut token_stack), one_product_three);


        let mut token_stack = TokenStack {
            tokens: Cell::new(vec![
                Token { t_type: TokenType::Number, value: "5".to_string() },
                Token { t_type: TokenType::Asterisk, value: "*".to_string() },
                Token { t_type: TokenType::Number, value: "3".to_string() },
                Token { t_type: TokenType::Asterisk, value: "*".to_string() },
                Token { t_type: TokenType::Number, value: "1".to_string() },
            ])
        };
        let expected = Node {
            op_type: OpType::Asterisk,
            token: Token {
                t_type: TokenType::Asterisk,
                value: "*".to_string(),
            },
            args: vec![
                one_product_three,
                Node {
                    op_type: OpType::Number,
                    token: Token {
                        t_type: TokenType::Number,
                        value: "5".to_string(),
                    },
                    args: vec![],
                },
            ],
        };
        assert_eq!(mul(&mut token_stack), expected);
    }

    #[test]
    fn test_priority() {
        let mut token_stack = TokenStack {
            tokens: Cell::new(vec![
                Token { t_type: TokenType::Number, value: "1".to_string() },
            ])
        };
        let expected = Node {
            op_type: OpType::Number,
            token: Token {
                t_type: TokenType::Number,
                value: "1".to_string(),
            },
            args: vec![],
        };
        assert_eq!(priority(&mut token_stack).unwrap(), expected);

        let mut token_stack = TokenStack {
            tokens: Cell::new(vec![
                Token { t_type: TokenType::Plus, value: "+".to_string() },
                Token { t_type: TokenType::Number, value: "2".to_string() },
            ])
        };
        let expected = Node {
            op_type: OpType::Number,
            token: Token {
                t_type: TokenType::Number,
                value: "2".to_string(),
            },
            args: vec![],
        };
        assert_eq!(priority(&mut token_stack).unwrap(), expected);


        let mut token_stack = TokenStack {
            tokens: Cell::new(vec![
                Token { t_type: TokenType::ParenthesisRight, value: ")".to_string() },
                Token { t_type: TokenType::Number, value: "2".to_string() },
                Token { t_type: TokenType::Plus, value: "+".to_string() },
                Token { t_type: TokenType::Number, value: "2".to_string() },
                Token { t_type: TokenType::ParenthesisLeft, value: "(".to_string() },
            ])
        };

        let expected = Node {
            op_type: OpType::Plus,
            token: Token {
                t_type: TokenType::Plus,
                value: "+".to_string(),
            },
            args: vec![
                Node {
                    op_type: OpType::Number,
                    token: Token {
                        t_type: TokenType::Number,
                        value: "2".to_string(),
                    },
                    args: vec![],
                },
                Node {
                    op_type: OpType::Number,
                    token: Token {
                        t_type: TokenType::Number,
                        value: "2".to_string(),
                    },
                    args: vec![],
                }
            ],
        };
        assert_eq!(priority(&mut token_stack).unwrap(), expected);
    }
}