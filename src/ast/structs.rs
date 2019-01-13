#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinType {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Value(BuiltinType),
    Symbol(String),
    Cons(Box<Expr>, Box<Expr>),
    Nil,
}