#[derive(Debug)]
pub enum Token {
    // types
    Num(String),
    Bool(bool),
    Str(String),
    // assign
    Assign,
    AddAssign,
    SubAssign,
    Incr,
    Decr,
    // arithmetic
    Add,
    Sub,
    Mult,
    Div,
    Pow,
    // bool ops
    Not,
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Lesser,
    LesserEquals,
    // other
    Keyword(String),
    Comment(String),
}