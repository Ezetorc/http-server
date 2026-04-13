use std::sync::Arc;

use crate::{
    http::{
        request::request::Request,
        response::{response::Response, status::Status},
        routing::router::Router,
    },
    server::{error::ServerError, server::Server},
};

mod http;
mod server;

fn get_juan(request: Request) -> Response {
    println!("get juan {request}");

    Response::new(Status::Ok)
        .with_body("hola".into())
        .with_header("x-lol", "tremenedo")
}

fn main() {
    let mut server: Server = Server::new("127.0.0.1", "8080");
    let mut users_router: Router = Router::new("/users");

    users_router.on_get("/juan", get_juan);

    server.route(users_router);

    let server: Arc<Server> = Arc::new(server);
    let result: Result<(), ServerError> = server.start();

    match result {
        Ok(()) => println!("# Server stopped #"),
        Err(error) => println!("{error}"),
    }
}
