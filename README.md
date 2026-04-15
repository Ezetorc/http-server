# Flow

1. Listen to connections
2. Receive connection
3. Read connection
4. Parse connection to HTTP Request object
5. Handle request with correspondent handler
6. Return a response to the client

# Entities

Server: Listen and read connections
HttpParser: Parse a string, converting it into an Request
Request: Represents an HTTP request
- Method
- Path
- Headers
- HTTP Version
- Body (optional)
Response: Represents an HTTP response
- Methods to build response
- When received by x function, it would transform itself into plain text,
  and then into bytes, to be retrieved to the client
  HttpRouter: Provides methods to abstract the "method & path -> handler" routing

# Entity Flow

0. User sets routes via HttpRouter
1. Server reads a connection
2. Server converts connection's bytes to string, getting headers and body (optional)
3. HttpParser converts string to Request
4. Server matches Request's method and path to pre-settled handler (if not found, returns 404 Not Found)
5. Matched handler uses the request to create a response
6. Server retrieves Response converted into bytes

# Path parsing

1) Divide path and query
> Found: /users/123?detailed=true&lol=12
> Result: users/123 | detailed=true&lol=12

2) Divide path in segments
> Found: /users/123
> Result: "users", "123"

3) Compare path segments and route segments length
> Route: /users/:id     > 2 Segments
> Found: /users/123/jaja > 3 Segments
> Result: False (not same amount of segments)

4) Compare segments
```rust
for segment in segments:
  if routerSegment starts_with ":":
    save_as_param(segment)
  else:
    if segment != routerSegment:
        continue
```

5) Parse query params
> Found: detailed=true&lolazo=12
> Result: { "detailed": "true", "lolazo": "12" }

6) Save in request
> request.set_query_parameters(query_parameters)
> request.set_path_parameters(path_parameters)

7) If NO route matched, return 404 Not found

# Initial Use Concept

```rust
fn get_user_by_id(request: Request) -> Response {
    let id_str = match request.get_parameter("id") {
        Some(id) => id,
        None => return Response::bad_request(),
    };

    let id: u32 = match id_str.parse() {
        Ok(id) => id,
        Err(_) => return Response::bad_request(),
    };

    Response::new(HttpStatus::Ok).with_body().with_headers()
}

let server = Server::new(port, address);
let users_route = HttpRouter::new("/users");

users_route.get("/:id", get_user_by_id);

server.route(users_route);

server.start();
```