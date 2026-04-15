use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use crate::{
    http::{
        content::{body::Body, headers::Headers, version::Version},
        parser::{buffer::Buffer, parser::Parser},
        request::{method::Method, request::Request, request_line::RequestLine},
        response::{handler_result::HandlerResult, response::Response, status::Status},
        routing::router::Router,
    },
    server::error::ServerError,
};

pub struct Server {
    pub port: String,
    pub host: String,
    routers: Vec<Router>,
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
                Ok(mut stream) => {
                    let server: Arc<Self> = Arc::clone(&self);

                    thread::spawn(move || match server.handle_connection(&mut stream) {
                        Ok(response) => {
                            if let Err(error) = stream.write_all(response.as_bytes().as_slice()) {
                                eprintln!("Write error: {:?}", error);
                            }
                        }
                        Err(error) => {
                            eprintln!("Connection error: {:?}", error);
                        }
                    });
                }
                Err(error) => {
                    eprintln!("Error accepting connection: {:?}", error);
                }
            }
        }
        Ok(())
    }

    fn handle_connection(&self, stream: &mut TcpStream) -> Result<Response, ServerError> {
        let mut request: Request = Self::build_request_from_stream(stream)?;
        let (path, queries) = request.split_path();
        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let Some(base_path) = segments.first() else {
            return Ok(Response::new(Status::NotFound));
        };

        let Some(router) = self.get_router(base_path) else {
            return Ok(Response::new(Status::NotFound));
        };

        let method: Method = request.method();
        let sub_segments: &[&str] = &segments[1..];
        if let Some((request_handler, parameters)) = router.get_handler(method, sub_segments) {
            let version: Version = request.version();
            let query_parameters: HashMap<String, String> = Parser::parse_query_parameters(queries);

            request.set_query_parameters(query_parameters);
            request.set_path_parameters(parameters);

            let result: HandlerResult = request_handler(request);

            match result {
                Ok(mut response) => {
                    response.set_version(version);

                    Ok(response)
                }
                Err(error) => Ok(error),
            }
        } else {
            return Ok(Response::new(Status::NotFound));
        }
    }

    pub fn route(&mut self, router: Router) -> &Self {
        self.routers.push(router);

        self
    }

    fn get_router(&self, base_path: &str) -> Option<&Router> {
        self.routers
            .iter()
            .find(|router| router.path() == base_path)
    }

    fn build_request_from_stream(stream: &mut TcpStream) -> Result<Request, ServerError> {
        let mut buffer: Buffer = Buffer::new();
        let mut chunk: [u8; Self::CHUNK_BYTES_AMOUNT] = [0; Self::CHUNK_BYTES_AMOUNT];

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
                let http_headers: Headers = Parser::parse_headers(headers)?;
                let body_length: usize = http_headers.get_as(Headers::CONTENT_LENGTH).unwrap_or(0);

                if body.len() < body_length {
                    continue;
                }

                let http_request_line: RequestLine = Parser::parse_request_line(request_line)?;
                let http_body: Option<Body> = Parser::parse_body(body)?;

                return Ok(Request::from(http_request_line, http_headers, http_body));
            }
        }
    }

    fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
