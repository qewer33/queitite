use std::{error::Error, fmt::Display, io, str::FromStr};

use crate::{evaluator::value::Value, lexer::cursor::Cursor};

pub type EvalResult<T> = std::result::Result<T, RuntimeEvent>;

#[derive(Debug)]
pub enum RuntimeEvent {
    Err(RuntimeErr),
    Return(Value),
    UserErr { val: Value, cursor: Cursor },
    Break,
    Continue,
}

impl RuntimeEvent {
    pub fn error(kind: ErrKind, msg: String, cursor: Cursor) -> Self {
        RuntimeEvent::Err(RuntimeErr {
            kind,
            msg,
            cursor,
            note: None,
        })
    }

    pub fn error_with_note(kind: ErrKind, msg: String, note: String, cursor: Cursor) -> Self {
        RuntimeEvent::Err(RuntimeErr {
            kind,
            msg,
            cursor,
            note: Some(note),
        })
    }

    pub fn user_err(val: Value, cursor: Cursor) -> Self {
        RuntimeEvent::UserErr { val, cursor }
    }

    pub fn is_break(&self) -> bool {
        matches!(self, RuntimeEvent::Break)
    }
    pub fn is_continue(&self) -> bool {
        matches!(self, RuntimeEvent::Continue)
    }
    pub fn is_return(&self) -> bool {
        matches!(self, RuntimeEvent::Return(_))
    }
}

impl From<io::Error> for RuntimeEvent {
    fn from(err: io::Error) -> Self {
        RuntimeEvent::error(ErrKind::IO, format!("IO error: {}", err), Cursor::new())
    }
}

#[derive(Debug)]
pub struct RuntimeErr {
    /// Error kind
    pub kind: ErrKind,
    /// Error message
    pub msg: String,
    /// Error location as a Cursor
    pub cursor: Cursor,
    /// Friendly note for the user
    pub note: Option<String>,
}

impl RuntimeErr {
    pub fn new(kind: ErrKind, msg: String, cursor: Cursor) -> Self {
        Self {
            kind,
            msg,
            cursor,
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
}

impl Error for RuntimeErr {}

impl Display for RuntimeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug)]
pub enum ErrKind {
    Type,
    Name,
    Arity,
    Value,
    Native,
    IO,
}

impl ToString for ErrKind {
    fn to_string(&self) -> String {
        match self {
            ErrKind::Type => "TypeErr",
            ErrKind::Name => "NameErr",
            ErrKind::Arity => "ArityErr",
            ErrKind::Value => "ValueErr",
            ErrKind::Native => "NativeErr",
            ErrKind::IO => "IOErr",
        }
        .into()
    }
}

impl FromStr for ErrKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <ErrKind as FromStr>::Err> {
        match s {
            "TypeErr" => Ok(ErrKind::Type),
            "NameErr" => Ok(ErrKind::Name),
            "ArityErr" => Ok(ErrKind::Arity),
            "ValueErr" => Ok(ErrKind::Value),
            "NativeErr" => Ok(ErrKind::Native),
            "IOErr" => Ok(ErrKind::IO),

            _ => Err(()),
        }
    }
}
