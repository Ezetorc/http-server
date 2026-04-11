use crate::server::{server::Server, server_error::ServerError};

mod http;
mod server;

fn main() {
    let server: Server = Server::new("127.0.0.1", "8080");

    let result: Result<(), ServerError> = server.start();

    match result {
        Ok(()) => println!("# Server stopped #"),
        Err(error) => println!("{error}"),
    }
}
