#[derive(Debug, PartialEq)]
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
    Identifier(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    EOL,
    EOF
}

pub const KEYWORDS: &[&str] = &[
    "do",
    "end",
    "if",
    "for",
    "while",
    "return",
    "yeet",
    "throw",
    "amogus"
];