use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use crate::{
    http::{
        content::{body::HttpBody, headers::HttpHeaders, version::HttpVersion},
        parser::{buffer::HttpBuffer, parser::HttpParser},
        request::{
            request::HttpRequest, request_handler::HttpRequestHandler,
            request_line::HttpRequestLine,
        },
        response::response::HttpResponse,
        routing::router::HttpRouter,
    },
    server::error::HttpServerError,
};

pub struct HttpServer {
    pub port: String,
    pub host: String,
    routers: Vec<HttpRouter>,
}

impl HttpServer {
    pub const CHUNK_BYTES_AMOUNT: usize = 1024;

    pub fn new(host: &str, port: &str) -> Self {
        Self {
            host: String::from(host),
            port: String::from(port),
            routers: Vec::new(),
        }
    }

    pub fn start(self: Arc<Self>) -> Result<(), HttpServerError> {
        let address: String = self.get_address();
        let listener: TcpListener = TcpListener::bind(&address)?;

        println!("# Server is listening");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server: Arc<Self> = Arc::clone(&self);

                    thread::spawn(move || {
                        let _ = server.handle_connection(stream);
                    });
                }
                Err(error) => {
                    eprintln!("Error accepting connection: {:?}", error);
                }
            }
        }
        Ok(())
    }

    pub fn route(&mut self, router: HttpRouter) -> &Self {
        self.routers.push(router);

        self
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), HttpServerError> {
        let request: HttpRequest = Self::build_request_from_stream(&mut stream)?;
        let (base_path, rest_path) = request.split_path();
        let matched_router: Option<&HttpRouter> = self.get_matched_router(base_path);

        if let Some(router) = matched_router {
            let matched_handler: Option<HttpRequestHandler> =
                router.get_matched_handler(request.get_method(), rest_path);

            if let Some(handler) = matched_handler {
                let version: HttpVersion = request.get_version();
                let mut response: HttpResponse = handler(request);

                response.set_version(version);

                stream.write_all(response.as_bytes().as_slice())?;
            }
        }

        Ok(())
    }

    fn get_matched_router(&self, base_path: &str) -> Option<&HttpRouter> {
        self.routers
            .iter()
            .find(|router| router.get_base_path() == base_path)
    }

    fn build_request_from_stream(stream: &mut TcpStream) -> Result<HttpRequest, HttpServerError> {
        let mut buffer: HttpBuffer = HttpBuffer::new();
        let mut chunk: [u8; Self::CHUNK_BYTES_AMOUNT] = [0; Self::CHUNK_BYTES_AMOUNT];

        loop {
            let bytes_read: usize = stream.read(&mut chunk)?;

            if bytes_read == 0 {
                break Err(HttpServerError::EmptyRequest);
            }

            buffer.add(&chunk[..bytes_read]);

            if buffer.exceeded_max_bytes() {
                return Err(HttpServerError::OutOfBonds);
            }

            if let Some((request_line, headers, body)) = buffer.split() {
                let http_headers: HttpHeaders = HttpParser::parse_headers(headers)?;

                let content_length: usize = http_headers
                    .get("content-length")
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(0);

                if body.len() < content_length {
                    continue;
                }

                let http_body: Option<HttpBody> = HttpParser::parse_body(body)?;

                let http_request_line: HttpRequestLine =
                    HttpParser::parse_request_line(request_line)?;

                return Ok(HttpRequest::from(
                    http_request_line,
                    http_headers,
                    http_body,
                ));
            }
        }
    }

    fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
