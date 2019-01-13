use crate::tokenizer::tokenizer::Token;
use std::fmt::Display;
use crate::tokenizer::tokenizer;

pub trait SuffixedType {
    fn get_suffix() -> &'static str;
    fn get_token(self) -> Token;
}

impl SuffixedType for i8 {
    fn get_suffix() -> &'static str {
        "i8"
    }

    fn get_token(self) -> Token {
        Token::Int8(self)
    }
}

impl SuffixedType for i16 {
    fn get_suffix() -> &'static str {
        "i16"
    }

    fn get_token(self) -> Token {
        Token::Int16(self)
    }
}

impl SuffixedType for i32 {
    fn get_suffix() -> &'static str {
        "i32"
    }

    fn get_token(self) -> Token {
        Token::Int32(self)
    }
}

impl SuffixedType for i64 {
    fn get_suffix() -> &'static str {
        "i64"
    }

    fn get_token(self) -> Token {
        Token::Int64(self)
    }
}

impl SuffixedType for f32 {
    fn get_suffix() -> &'static str {
        "f32"
    }

    fn get_token(self) -> Token {
        Token::Float32(self)
    }
}


impl SuffixedType for f64 {
    fn get_suffix() -> &'static str {
        "f64"
    }

    fn get_token(self) -> Token {
        Token::Float64(self)
    }
}

pub fn test_suffix_helper<T>(val: T)
    where T: Display + SuffixedType {
    let program = format!("( setq symbol {}{} )", val, T::get_suffix());
    let tokenizer= tokenizer::Tokenizer::new(program);
    let expected = [
        Token::OpenParen,
        Token::Symbol("setq".to_string()),
        Token::Symbol("symbol".to_string()),
        val.get_token(),
        Token::CloseParen,
    ];
    check_correctness_helper(&expected, &tokenizer);
}

pub fn check_correctness_helper(expected : &[Token], tokenizer : &tokenizer::Tokenizer) {
    assert_eq!(tokenizer.tokens().count(), expected.len(), "Exact and expected lengths not equal");
    for (r, e) in tokenizer.tokens().zip(expected.iter()) {
        assert_eq!(r, *e, "Tokens not equal");
    }
}