use std::fmt;

use crate::http::http_error::HttpError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod {
    Options,
    Connect,
    Delete,
    Trace,
    Patch,
    Head,
    Post,
    Get,
    Put,
}

impl TryFrom<&str> for HttpMethod {
    type Error = HttpError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            "put" => Ok(Self::Put),
            "delete" => Ok(Self::Delete),
            "patch" => Ok(Self::Patch),
            "head" => Ok(Self::Head),
            "options" => Ok(Self::Options),
            "connect" => Ok(Self::Connect),
            "trace" => Ok(Self::Trace),
            _ => Err(HttpError::InvalidMethod(value.to_string())),
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Patch => "PATCH",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
            Self::Connect => "CONNECT",
            Self::Trace => "TRACE",
        };

        write!(formatter, "{}", method)
    }
}
