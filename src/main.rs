mod tokenizer;
mod parser;
pub use self::tokenizer::Tokenizer;

fn main() {
    let tokenizer = Tokenizer::new("( lel 123 )".to_string());

    for t in tokenizer.tokens() {
        print!("{:?}\n", t);
    }
}
