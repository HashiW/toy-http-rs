use std::net::SocketAddr;

use std::collections::HashMap;

use http_lib::http::{HttpMethod, HttpStatusCode};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;


use http_lib::request::*;

use http_lib::response::*;

use http_lib::server::*;

fn hello_handler(_req: Request) -> FutureResponse<'static> {
    let html = "<html><body><h1>Hello, world!</h1></body></html>";
    let response = Response {
        version: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        headers: {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/html".to_string());
            headers
        },
        body: Some(html.to_string()),
    };
    Box::pin(async move { Ok(response) })
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let server = ServerBuilder::new()
        .bind(addr)
        .route("/",HttpMethod::GET, hello_handler)
        .build()?
        .run()
        .await?;

    println!("Hello, world!");
    Ok(())
}
