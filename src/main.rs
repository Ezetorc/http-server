use std::sync::Arc;

use crate::{
    http::{http_request::HttpRequest, http_response::HttpResponse, http_router::HttpRouter},
    server::{server::Server, server_error::ServerError},
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
