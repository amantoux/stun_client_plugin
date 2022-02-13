use std::fmt::{Debug, Display};

pub enum Error {
    DefaultError(String),
    ParseError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DefaultError(description) => return write!(f, "Default error : {}", description),
            Self::ParseError => return write!(f, "Parse error"),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DefaultError(description) => return write!(f, "Default error : {}", description),
            Self::ParseError => return write!(f, "Parse error"),
        }
    }
}

impl std::error::Error for Error {}
