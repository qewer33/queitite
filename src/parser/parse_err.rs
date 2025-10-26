use std::{
    error::Error,
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

use crate::lexer::cursor::Cursor;

pub type ParseResult<T> = std::result::Result<T, ParseErr>;

#[derive(Debug, Clone)]
pub struct ParseErr {
    /// Error message
    pub msg: String,
    /// Error location as a Cursor
    pub cursor: Cursor,
    /// Expected token lexeme
    pub expected: Option<String>,
    /// Found token lexeme
    pub found: Option<String>,
    /// Friendly note for the user
    pub note: Option<String>,
}

impl ParseErr {
    pub fn new(msg: String, cursor: Cursor) -> Self {
        Self {
            msg,
            cursor,
            expected: None,
            found: None,
            note: None,
        }
    }

    pub fn with_context(
        msg: String,
        cursor: Cursor,
        expected: Option<String>,
        found: Option<String>,
    ) -> Self {
        Self {
            msg,
            cursor,
            expected,
            found,
            note: None,
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

    pub fn expected(mut self, expected: String) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn found(mut self, found: String) -> Self {
        self.found = Some(found);
        self
    }
}

impl Error for ParseErr {}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<ParseIntError> for ParseErr {
    fn from(_value: ParseIntError) -> Self {
        Self::new("".into(), Cursor::new())
    }
}

impl From<ParseFloatError> for ParseErr {
    fn from(_value: ParseFloatError) -> Self {
        Self::new("".into(), Cursor::new())
    }
}

impl From<()> for ParseErr {
    fn from(_value: ()) -> Self {
        Self::new("".into(), Cursor::new())
    }
}
