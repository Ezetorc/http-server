use std::sync::Arc;

use crate::{
    http::{
        request::request::HttpRequest, response::response::HttpResponse,
        routing::router::HttpRouter,
    },
    server::{error::ServerError, server::Server},
};

mod http;
mod server;

fn get_juan(request: HttpRequest) -> HttpResponse {
    println!("get juan {request}");

    HttpResponse { hola: true }
}

fn main() {
    let mut server: Server = Server::new("127.0.0.1", "8080");
    let mut users_router: HttpRouter = HttpRouter::new("/users");

    users_router.on_get("/juan", get_juan);

    server.route(users_router);

    let server: Arc<Server> = Arc::new(server);
    let result: Result<(), ServerError> = server.start();

    match result {
        Ok(()) => println!("# Server stopped #"),
        Err(error) => println!("{error}"),
    }
}
