use std::{fmt, io::Error, string::FromUtf8Error};

use crate::http::error::HttpError;

#[derive(Debug)]
pub enum HttpServerError {
    InputOutput(Error),
    OutOfBonds,
    InvalidUtf8(FromUtf8Error),
    HttpError(HttpError),
    EmptyRequest,
}

impl fmt::Display for HttpServerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOutput(error) => write!(formatter, "Input/Output Error: {}", error),
            Self::InvalidUtf8(error) => write!(formatter, "Invalid UTF8: {error}"),
            Self::HttpError(error) => write!(formatter, "HTTP Error: {error}"),
            Self::EmptyRequest => write!(formatter, "Request is empty"),
            Self::OutOfBonds => write!(formatter, "Request size is out of bounds"),
        }
    }
}

impl From<HttpError> for HttpServerError {
    fn from(value: HttpError) -> Self {
        Self::HttpError(value)
    }
}

impl From<FromUtf8Error> for HttpServerError {
    fn from(value: FromUtf8Error) -> Self {
        Self::InvalidUtf8(value)
    }
}

impl From<Error> for HttpServerError {
    fn from(value: Error) -> Self {
        Self::InputOutput(value)
    }
}
