use std::{fmt, string::FromUtf8Error};

#[derive(Debug)]
pub enum HttpError {
    InvalidMethod(String),
    InvalidVersion(String),
    InvalidParse(FromUtf8Error),
}

impl fmt::Display for HttpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMethod(method) => write!(formatter, "Method '{method}' is invalid"),
            Self::InvalidParse(error) => write!(formatter, "Error during parsing: {error}"),
            Self::InvalidVersion(error) => write!(formatter, "HTTP Version '{error}' is invalid"),
        }
    }
}

impl From<FromUtf8Error> for HttpError {
    fn from(value: FromUtf8Error) -> Self {
        Self::InvalidParse(value)
    }
}
