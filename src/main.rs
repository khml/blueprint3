use std::io::Write;

mod token;
mod parser;
mod evaluate;

use token::tokenize;

fn main() {
    loop {
        let mut sentence = String::new();
        print!("> ");
        std::io::stdout().flush().ok();

        std::io::stdin().read_line(&mut sentence).ok();

        let tokens = tokenize(sentence.trim()).unwrap();
        println!("{:?}", tokens);
        let mut token_stack = parser::TokenStack::new(tokens);

        let root_node = parser::parse(&mut token_stack);
        println!("{:?}", root_node);

        let val = evaluate::evaluate(&root_node);
        println!("{}", val);

        println!();
    }
}
