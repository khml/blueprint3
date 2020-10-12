use std::io::Write;

mod token;
mod parser;
mod evaluate;
mod context;

use token::tokenize;

fn main() {
    let mut context = context::Context::new();
    loop {
        let mut sentence = String::new();
        print!("> ");
        std::io::stdout().flush().ok();

        std::io::stdin().read_line(&mut sentence).ok();

        let tokens = tokenize(sentence.trim()).unwrap();

        if tokens.len() == 0 {
            continue;
        }

        println!("{:?}", tokens);

        let mut token_stack = parser::TokenStack::new(tokens);

        let root_node = parser::parse(&mut token_stack);
        println!("{:?}", root_node);

        let val = evaluate::evaluate(&root_node, &mut context);
        match val {
            Some(val) => { println!("{}", val); }
            _ => { println!(); }
        }

        println!();
    }
}
