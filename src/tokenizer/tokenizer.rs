use std::iter::Iterator;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Str(String),
    Symbol(String),
    OpenParen,
    CloseParen,
    Invalid,
}

pub struct Tokenizer {
    buf : String,
}

impl Tokenizer {
    pub fn new(s : String) -> Tokenizer {
        Tokenizer { buf: s }
    }

    pub fn tokens(&self) -> TokenIterator {
        let chars = self.buf.chars();
        TokenIterator {buf: chars}
    }
}

pub struct TokenIterator<'a> {
    buf : Chars<'a>
}

impl<'a> TokenIterator<'a> {
    fn parse_num(&mut self, c: char) -> Option<Token> {
        let mut s = c.to_string();
        for c in &mut self.buf {
            match c {
                c if c.is_digit(10) => s += &c.to_string(),
                ' ' | '\n' | '\r' | '\t' | ')' => return Some(Token::Int32(s.parse::<i32>().unwrap())),
                // TODO: add prefixed integers and floating point numbers
                _ => return Some(Token::Invalid),
            }
        }
        None
    }

    fn parse_str(&mut self) -> Option<Token> {
        let mut s = "".to_string();
        let mut close = false;
        for c in &mut self.buf {
            match c {
                '\"' => close = true,
                ' ' | '\n' | '\r' | '\t' | ')' =>
                    return if close {
                        Some(Token::Str(s))
                    } else {
                        Some(Token::Invalid)
                    },
                _ if close => return Some(Token::Invalid),
                c => s += &c.to_string(),
            }
        }
        None
    }

    fn parse_symbol(&mut self, c: char) -> Option<Token> {
        let mut s = c.to_string();
        for c in &mut self.buf {
            match c {
                ' ' | '\n' | '\r' | '\t' | ')' => return Some(Token::Symbol(s)),
                c if c.is_alphanumeric() => s += &c.to_string(),
                _ => return Some(Token::Invalid),
            }
        }
        None
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        for c in &mut self.buf {
            match c {
                ' ' | '\n' | '\r' | '\t' => continue,
                '(' => return Some(Token::OpenParen),
                ')' => return Some(Token::CloseParen),
                ch if ch.is_digit(10) => return self.parse_num(c),
                '\"' => return self.parse_str(),
                ch if ch.is_alphabetic() => return self.parse_symbol(c),
                _ => return Some(Token::Invalid),
            }
        }
        return None
    }
}





