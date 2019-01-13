use crate::tokenizer::{TokenIterator, Token};
use crate::ast::*;

pub fn parse(tokens : &mut TokenIterator) -> Box<Expr> {
    match tokens.next() {
        Some(Token::Int8(v)) => Box::new(
            Expr::Cons(
                Box::new(Expr::Value(BuiltinType::Int8(v))), parse(tokens))),
        Some(Token::Int16(v)) => Box::new(
            Expr::Cons(
                Box::new(Expr::Value(BuiltinType::Int16(v))), parse(tokens))),
        Some(Token::Int32(v)) => Box::new(
            Expr::Cons(
                Box::new(Expr::Value(BuiltinType::Int32(v))), parse(tokens))),
        Some(Token::Int64(v)) => Box::new(
            Expr::Cons(
                Box::new(Expr::Value(BuiltinType::Int64(v))), parse(tokens))),
        Some(Token::Float32(v)) => Box::new(
            Expr::Cons(
                Box::new(Expr::Value(BuiltinType::Float32(v))), parse(tokens))),
        Some(Token::Float64(v)) => Box::new(
            Expr::Cons(
                Box::new(Expr::Value(BuiltinType::Float64(v))), parse(tokens))),
        Some(Token::Str(v)) => Box::new(
            Expr::Cons(
                Box::new(Expr::Value(BuiltinType::Str(v))), parse(tokens))),
        Some(Token::Symbol(v)) => Box::new(
            Expr::Cons(
                Box::new(Expr::Symbol(v)), parse(tokens))),
        Some(Token::OpenParen) => Box::new(
            Expr::Cons(parse(tokens), parse(tokens))),
        Some(Token::CloseParen) => Box::new(Expr::Nil),
        Some(Token::Invalid) => panic!("AST: Invalid token found"),
        None => Box::new(Expr::Nil),
    }
}