use std::{error::Error, fmt::Display};

use crate::{evaluator::value::Value, lexer::cursor::Cursor};

pub type EvalResult<T> = std::result::Result<T, RuntimeErr>;

#[derive(Debug)]
pub struct RuntimeErr {
    /// Error message
    pub msg: String,
    /// Error location as a Cursor
    pub cursor: Cursor,
    /// Friendly note for the user
    pub note: Option<String>,
    /// For return statement
    pub return_val: Option<Value>
}

impl RuntimeErr {
    pub fn new(msg: String, cursor: Cursor) -> Self {
        Self { msg, cursor, note: None, return_val: None }
    }

    pub fn return_val(val: Value) -> Self {
        Self {
            msg: String::new(),
            cursor: Cursor::new(),
            note: None,
            return_val: Some(val)
        }
    }

    pub fn msg(mut self, msg: String) -> Self {
        self.msg = msg;
        self
    }

    pub fn cursor(mut self, cursor: Cursor) -> Self {
        self.cursor = cursor;
        self
    }

    pub fn note(mut self, note: String) -> Self {
        self.note = Some(note);
        self
    }
}

impl Error for RuntimeErr {}

impl Display for RuntimeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
