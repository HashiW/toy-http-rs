macro_rules! make_route {
    ($method:ident, $handler:expr) => {
        fn $method() -> Route {
            let method = match stringify!($method).to_uppercase().as_str() {
                "GET" => HttpMethod::GET,
                "POST" => HttpMethod::POST,
                "PUT" => HttpMethod::PUT,
                "DELETE" => HttpMethod::DELETE,
                "HEAD" => HttpMethod::HEAD,
                "OPTIONS" => HttpMethod::OPTIONS,
                "CONNECT" => HttpMethod::CONNECT,
                "TRACE" => HttpMethod::TRACE,
                "PATCH" => HttpMethod::PATCH,
                _ => HttpMethod::OTHER(stringify!($method).to_string()),
            };
            Route {
                method,
                handler: $handler,
            }
        }
    };
}