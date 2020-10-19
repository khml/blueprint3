use std::io::Write;
use std::env;

mod token;
mod parser;
mod evaluate;
mod context;
mod file;

use token::tokenize;
use crate::parser::Node;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        repl()
    }

    let mut nodes: Vec<Node> = vec![];
    println!("{:?}", &args);
    for sentence in file::read_txt(&args[1]).unwrap() {
        let tokens = tokenize(sentence.trim()).unwrap();
        if tokens.len() == 0 { continue; }
        let mut token_stack = parser::TokenStack::new(tokens);
        nodes.push(parser::parse(&mut token_stack));
    }

    let mut context = context::Context::new();
    for node in nodes {
        let val = evaluate::evaluate(&node, &mut context);
        match val {
            Some(val) => { println!("{}", val); }
            None => {}
        }
    }
}

fn repl() {
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
            _ => {}
        }

        println!();
    }
}
