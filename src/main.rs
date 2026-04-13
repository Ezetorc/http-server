use std::sync::Arc;

use crate::{
    http::{
        request::request::HttpRequest,
        response::{response::HttpResponse, status::HttpStatus},
        routing::router::HttpRouter,
    },
    server::{error::HttpServerError, server::HttpServer},
};

mod http;
mod server;

fn get_juan(request: HttpRequest) -> HttpResponse {
    println!("get juan {request}");

    HttpResponse::new(HttpStatus::Ok)
        .with_body("hola".into())
        .with_header("x-lol", "tremenedo")
}

fn main() {
    let mut server: HttpServer = HttpServer::new("127.0.0.1", "8080");
    let mut users_router: HttpRouter = HttpRouter::new("/users");

    users_router.on_get("/juan", get_juan);

    server.route(users_router);

    let server: Arc<HttpServer> = Arc::new(server);
    let result: Result<(), HttpServerError> = server.start();

    match result {
        Ok(()) => println!("# Server stopped #"),
        Err(error) => println!("{error}"),
    }
}
