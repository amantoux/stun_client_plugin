use std::fmt::{Debug, Display};

pub enum Error {
    Default(String),
    Binding(String),
    Connect(String),
    Send(String),
    Receive(String),
    Parse,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(description) => return write!(f, "Default error : {}", description),
            Self::Binding(description) => return write!(f, "Binding error : {}", description),
            Self::Connect(description) => return write!(f, "Connect error : {}", description),
            Self::Send(description) => return write!(f, "Send error : {}", description),
            Self::Receive(description) => return write!(f, "Receive error : {}", description),
            Self::Parse => return write!(f, "Parse error"),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(description) => return write!(f, "Default error : {}", description),
            Self::Binding(description) => return write!(f, "Binding error : {}", description),
            Self::Connect(description) => return write!(f, "Connect error : {}", description),
            Self::Send(description) => return write!(f, "Send error : {}", description),
            Self::Receive(description) => return write!(f, "Receive error : {}", description),
            Self::Parse => return write!(f, "Parse error"),
        }
    }
}

impl std::error::Error for Error {}
