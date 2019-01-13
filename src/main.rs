#![feature(box_patterns)]

mod tokenizer;
mod parser;
mod ast;

use crate::tokenizer::Tokenizer;

fn main() {
    let tokenizer = Tokenizer::new("( cons )".to_string());
    let root = parser::parse(&mut tokenizer.tokens());
    print!("{:?}\n", root);
}
