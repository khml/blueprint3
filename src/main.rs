mod token;
mod parser;
mod evaluate;

use token::tokenize;

fn main() {
    let sentence = "1 + 2 + 3";

    let mut tokens = tokenize(sentence);
    let mut token_stack = parser::TokenStack::new(tokens);

    let root_node = parser::sum(&mut token_stack);
    println!("{:?}", root_node);

    let val = evaluate::evaluate(&root_node);
    println!("{}", val)
}
