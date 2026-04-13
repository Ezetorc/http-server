# Flow

1. Listen to connections
2. Receive connection
3. Read connection
4. Parse connection to HTTP Request object
5. Handle request with correspondent handler
6. Return a response to the client

# Entities

Server: Listen and read connections
HttpParser: Parse a string, converting it into an HttpRequest
HttpRequest: Represents an HTTP request
- Method
- Path
- Headers
- HTTP Version
- Body (optional)
HttpResponse: Represents an HTTP response
- Methods to build response
- When received by x function, it would transform itself into plain text,
  and then into bytes, to be retrieved to the client
  HttpRouter: Provides methods to abstract the "method & path -> handler" routing

# Entity Flow

0. User sets routes via HttpRouter
1. Server reads a connection
2. Server converts connection's bytes to string, getting headers and body (optional)
3. HttpParser converts string to HttpRequest
4. Server matches HttpRequest's method and path to pre-settled handler (if not found, returns 404 Not Found)
5. Matched handler uses the request to create a response
6. Server retrieves HttpResponse converted into bytes

# Use Concept

```rust
fn get_user_by_id(request: HttpRequest) -> HttpResponse {
    let id_str = match request.get_parameter("id") {
        Some(id) => id,
        None => return HttpResponse::bad_request(),
    };

    let id: u32 = match id_str.parse() {
        Ok(id) => id,
        Err(_) => return HttpResponse::bad_request(),
    };

    HttpResponse::new(HttpStatus::Ok).with_body().with_headers()
}

let server = Server::new(port, address);
let users_route = HttpRouter::new("/users");

users_route.get("/:id", get_user_by_id);

server.route(users_route);

server.start();
```