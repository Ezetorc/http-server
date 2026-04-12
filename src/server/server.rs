use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use crate::{
    http::{
        http_body::HttpBody, http_buffer::HttpBuffer, http_headers::HttpHeaders,
        http_parser::HttpParser, http_request::HttpRequest,
        http_request_handler::HttpRequestHandler, http_request_line::HttpRequestLine,
        http_router::HttpRouter,
    },
    server::server_error::ServerError,
};

pub struct Server {
    pub port: String,
    pub host: String,
    routers: Vec<HttpRouter>,
}

impl Server {
    pub const CHUNK_BYTES_AMOUNT: usize = 1024;

    pub fn new(host: &str, port: &str) -> Self {
        Self {
            host: String::from(host),
            port: String::from(port),
            routers: Vec::new(),
        }
    }

    pub fn start(self: Arc<Self>) -> Result<(), ServerError> {
        let address: String = self.get_address();
        let listener: TcpListener = TcpListener::bind(&address)?;

        println!("# Server is listening");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server: Arc<Server> = Arc::clone(&self);

                    thread::spawn(move || {
                        let _ = server.handle_request(stream);
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

    fn handle_request(&self, stream: TcpStream) -> Result<(), ServerError> {
        let request: HttpRequest = Server::build_request_from_stream(stream)?;

        println!("request: {request}");

        let request_base_path: String = request.get_base_path();
        let matched_router: Option<&HttpRouter> = self.get_matched_router(request_base_path);

        if let Some(router) = matched_router {
            let matched_handler: Option<HttpRequestHandler> =
                router.get_matched_handler(request.get_method(), &request.get_base_path());

            println!("matched_handler: {:?}", matched_handler);
        }

        Ok(())
    }

    fn get_matched_router(&self, base_path: String) -> Option<&HttpRouter> {
        self.routers
            .iter()
            .find(|router| router.get_base_path() == base_path)
    }

    fn build_request_from_stream(mut stream: TcpStream) -> Result<HttpRequest, ServerError> {
        let mut buffer: HttpBuffer = HttpBuffer::new();
        let mut chunk: [u8; Server::CHUNK_BYTES_AMOUNT] = [0; Server::CHUNK_BYTES_AMOUNT];

        loop {
            let bytes_read: usize = stream.read(&mut chunk)?;

            if bytes_read == 0 {
                break Err(ServerError::EmptyRequest);
            }

            buffer.add(&chunk[..bytes_read]);

            if buffer.exceeded_max_bytes() {
                return Err(ServerError::OutOfBonds);
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
