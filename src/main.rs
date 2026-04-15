use std::sync::Arc;

use crate::{
    http::{
        request::request::Request,
        response::{handler_result::HandlerResult, response::Response, status::Status},
        routing::router::Router,
    },
    server::{
        middleware::{middleware_result::MiddlewareResult, next_fn::NextFn},
        server::Server,
    },
};

mod http;
mod server;

// =======================
// 🧠 HANDLER
// =======================
fn get_user_by_id(request: Request) -> HandlerResult {
    let user_id: String = request.get_parameter_or_error("id")?;
    let auth: String = request.get_header_or_error("x-auth")?;

    println!("User ID: {user_id}");
    println!("Auth: {auth}");

    Ok(Response::new(Status::Ok)
        .with_json(format!("{{ \"user_id\": \"{}\" }}", user_id).as_str())
        .with_header("x-powered-by", "rust-server"))
}

// =======================
// 🔐 GLOBAL MIDDLEWARE
// =======================
fn auth_middleware(request: &mut Request, next: NextFn) -> MiddlewareResult {
    println!("[auth_middleware] running");

    // Simulate authentication
    request.set_header("x-auth", "authenticated-user");

    next()
}

// =======================
// 🧩 ROUTE MIDDLEWARE
// =======================
fn validate_id_middleware(request: &mut Request, next: NextFn) -> MiddlewareResult {
    println!("[validate_id_middleware] running");

    let id: String = request.get_parameter_or_error("id")?;

    if id.parse::<u32>().is_err() {
        return Err(
            Response::new(Status::BadRequest).with_json("{ \"error\": \"id must be numeric\" }")
        );
    }

    next()
}

// =======================
// 🚀 ENTRY POINT
// =======================
fn main() {
    let mut server: Server = Server::new("127.0.0.1", "8080");
    let mut users_router: Router = Router::new("/users");

    users_router
        .on_get("/:id", get_user_by_id)
        .with(Box::new(validate_id_middleware));
    
    server.use_middleware(Box::new(auth_middleware));
    server.route(users_router);

    let server: Arc<Server> = Arc::new(server);

    match server.start() {
        Ok(()) => println!("# Server stopped #"),
        Err(error) => println!("{error}"),
    }
}
