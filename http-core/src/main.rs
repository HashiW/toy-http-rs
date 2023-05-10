use std::net::SocketAddr;

use std::collections::HashMap;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;


macro_rules! say_hello {
    () => {
        println!("HELLO")
    };
}
use http_lib::request::*;

use http_lib::response::*;

use http_lib::server::*;


fn hello_handler(_req: &Request) -> FutureResponse {

let response = Response {
        version: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        headers: HashMap::new(),
        body: Some("Hello, world!".to_string()),
    };

    Ok(Box::pin(async move { response }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    say_hello!();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let server = ServerBuilder::new()
        .bind(addr)
        .route("/", hello_handler)
        .build()?.run().await?;


    println!("Hello, world!");
    Ok(())
}
