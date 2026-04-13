use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use crate::{
    http::{
        content::{body::Body, headers::Headers, version::Version},
        parser::{buffer::Buffer, parser::Parser},
        request::{
            method::Method, request::Request, request_handler::RequestHandler,
            request_line::RequestLine,
        },
        response::response::Response,
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

    pub fn route(&mut self, router: Router) -> &Self {
        self.routers.push(router);

        self
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), ServerError> {
        let request: Request = Self::build_request_from_stream(&mut stream)?;
        let (base_path, rest_path) = request.split_path();

        let Some(router) = self.get_router(base_path) else {
            return Ok(());
        };

        let method: Method = request.method();
        let request_handler: Option<RequestHandler> = router.get_handler(method, rest_path);

        let Some(handler) = request_handler else {
            return Ok(());
        };

        let version: Version = request.version();
        let mut response: Response = handler(request);
        response.set_version(version);

        stream.write_all(response.as_bytes().as_slice())?;

        Ok(())
    }

    fn get_router(&self, base_path: &str) -> Option<&Router> {
        self.routers
            .iter()
            .find(|router| router.base_path() == base_path)
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

                let content_length: usize = http_headers
                    .get("content-length")
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(0);

                if body.len() < content_length {
                    continue;
                }

                let http_body: Option<Body> = Parser::parse_body(body)?;

                let http_request_line: RequestLine = Parser::parse_request_line(request_line)?;

                return Ok(Request::from(http_request_line, http_headers, http_body));
            }
        }
    }

    fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
