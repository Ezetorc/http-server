use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{
    http::{
        http_body::HttpBody, http_buffer::HttpBuffer, http_headers::HttpHeaders,
        http_parser::HttpParser, http_request::HttpRequest, http_request_line::HttpRequestLine,
    },
    server::server_error::ServerError,
};

pub struct Server {
    pub port: String,
    pub host: String,
}

impl Server {
    pub const CHUNK_BYTES_AMOUNT: usize = 1024;

    pub fn new(host: &str, port: &str) -> Self {
        Self {
            host: String::from(host),
            port: String::from(port),
        }
    }

    pub fn start(&self) -> Result<(), ServerError> {
        let address: String = self.get_address();
        let listener: TcpListener = TcpListener::bind(&address)?;

        println!("# Server is listening");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| Server::handle_stream(stream));
                }
                Err(error) => {
                    eprintln!("Error accepting connection: {:?}", error);
                }
            }
        }

        Ok(())
    }

    fn handle_stream(mut stream: TcpStream) -> Result<(), ServerError> {
        let mut buffer: HttpBuffer = HttpBuffer::new();
        let mut chunk: [u8; Server::CHUNK_BYTES_AMOUNT] = [0; Server::CHUNK_BYTES_AMOUNT];

        loop {
            let bytes_read: usize = stream.read(&mut chunk)?;

            if bytes_read == 0 {
                break;
            }

            buffer.add(&chunk[..bytes_read]);

            if buffer.exceeded_max_bytes() {
                return Err(ServerError::OutOfBonds);
            }

            if let Some((request_line, headers, body)) = buffer.split() {
                let http_headers: HttpHeaders = HttpParser::parse_headers(headers)?;

                let content_length: usize = http_headers
                    .get(&"content-length".to_string())
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(0);

                if body.len() < content_length {
                    continue;
                }

                let http_body: Option<HttpBody> = HttpParser::parse_body(body)?;
                let http_request_line: HttpRequestLine =
                    HttpParser::parse_request_line(request_line)?;
                let request: HttpRequest =
                    HttpRequest::from(http_request_line, http_headers, http_body);

                println!("HttpRequest {request}");
            }
        }

        Ok(())
    }

    fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
