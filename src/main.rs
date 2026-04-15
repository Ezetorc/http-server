use std::sync::Arc;

use crate::{
    http::{
        request::request::Request,
        response::{handler_result::HandlerResult, response::Response, status::Status},
        routing::router::Router,
    },
    server::{
        error::ServerError,
        middleware::{middleware_result::MiddlewareResult, next_fn::NextFn},
        server::Server,
    },
};

mod http;
mod server;

fn get_user_by_id(request: Request) -> HandlerResult {
    let x_lol: String = request.get_header_or_error("x-auth")?;

    println!("get_user_by_id: {request} | {x_lol}");

    Ok(Response::new(Status::Ok)
        .with_json("{ 'chat': 'esto es real' }")
        .with_header("x-lol", "tremenedo"))
}

fn auth_middleware(request: &mut Request, next: NextFn) -> MiddlewareResult {
    request.set_header("x-auth", "la madafucking autenticacion");

    if true {
        next()
    } else {
        Err(Response::new(Status::Conflict))
    }
}

fn main() {
    let mut server: Server = Server::new("127.0.0.1", "8080");
    let mut users_router: Router = Router::new("/users");

    server.use_middleware(Box::new(auth_middleware));

    users_router.on_get("/:id", get_user_by_id);

    server.route(users_router);

    let server: Arc<Server> = Arc::new(server);
    let result: Result<(), ServerError> = server.start();

    match result {
        Ok(()) => println!("# Server stopped #"),
        Err(error) => println!("{error}"),
    }
}
