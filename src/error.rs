use std::fmt;

use fmt::{Display, Formatter};

#[derive(Debug)]
pub enum NoteError {
    Default,
    Message(String),
    IO(std::io::Error),
}

impl Display for NoteError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            NoteError::Default => write!(f, "Default NoteError"),
            NoteError::Message(ref message) => {
                write!(f, "NoteError: {}", message)
            }
            NoteError::IO(ref e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for NoteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            NoteError::Default => None,
            NoteError::Message(_) => None,
            NoteError::IO(ref e) => Some(e),
        }
    }
}

impl From<std::io::Error> for NoteError {
    fn from(err: std::io::Error) -> NoteError {
        NoteError::IO(err)
    }
}
