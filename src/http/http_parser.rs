use crate::http::{
    http_body::HttpBody, http_error::HttpError, http_headers::HttpHeaders,
    http_request_line::HttpRequestLine,
};

pub struct HttpParser;

impl HttpParser {
    pub fn parse_body(body: &[u8]) -> Result<Option<HttpBody>, HttpError> {
        if body.len() == 0 {
            return Ok(None);
        }

        let http_body: HttpBody = HttpBody::new(body);

        Ok(Some(http_body))
    }

    pub fn parse_request_line(request_line: &[u8]) -> Result<HttpRequestLine, HttpError> {
        let request_line_string: String = String::from_utf8(request_line.to_vec())?;
        let http_request_line: HttpRequestLine = HttpRequestLine::try_from(request_line_string)?;

        Ok(http_request_line)
    }

    pub fn parse_headers(headers: &[u8]) -> Result<HttpHeaders, HttpError> {
        let headers_string: String = String::from_utf8(headers.to_vec())?;
        let http_headers: HttpHeaders = HttpHeaders::from(headers_string);

        Ok(http_headers)
    }
}
