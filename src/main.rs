mod token;
use token::tokenize;

fn main() {
    let sentence = "a * b = c + d";
    for token in tokenize(sentence) {
        println!("{:?}", token)
    }
}
