use std::sync::Arc;

use crate::{
    http::{
        request::request::Request,
        response::{handler_result::HandlerResult, response::Response, status::Status},
        routing::router::Router,
    },
    server::{error::ServerError, server::Server},
};

mod http;
mod server;

fn get_user_by_id(request: Request) -> HandlerResult {
    let id: String = request.get_parameter_or_error("id")?;
    let lol: String = request.get_query_or_error("lol")?;

    println!("get_user_by_id: {request}:: {} {}", id, lol);

    Ok(Response::new(Status::Ok)
        .with_json("{ 'chat': 'esto es real' }")
        .with_header("x-lol", "tremenedo"))
}

fn get_juan_by_id(request: Request) -> HandlerResult {
    let id: String = request.get_parameter_or_error("id")?;
    let lol: String = request.get_query_or_error("lol")?;

    println!("get_juan_by_id: {request}:: {} {}", id, lol);

    Ok(Response::new(Status::Ok)
        .with_json("{ 'chat': 'esto es real' }")
        .with_header("x-lol", "tremenedo"))
}

fn main() {
    let mut server: Server = Server::new("127.0.0.1", "8080");
    let mut users_router: Router = Router::new("/users");

    users_router.on_get("/:id", get_user_by_id);
    users_router.on_get("/juan/:id", get_juan_by_id);

    server.route(users_router);

    let server: Arc<Server> = Arc::new(server);
    let result: Result<(), ServerError> = server.start();

    match result {
        Ok(()) => println!("# Server stopped #"),
        Err(error) => println!("{error}"),
    }
}
