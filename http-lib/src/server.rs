use crate::request::Request;
use crate::response::Response;

use crate::http::*;
use httparse;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::{fs, io::prelude::*, thread, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Sender};

pub type FutureResponse =
    Result<Pin<Box<dyn Future<Output = Response> + Send>>, Box<dyn std::error::Error + Send>>;

pub type RouteHandler = fn(&Request) -> FutureResponse;

pub struct Server {
    address: SocketAddr,
    routes: HashMap<Route, RouteHandler>,
}
#[derive(Eq, Hash, PartialEq,Clone)]
struct Route {
    method: HttpMethod,
    path: String,
}
impl Server {
    pub async fn run(&self) -> std::io::Result<()> {
        let address = self.address;
        let listener = TcpListener::bind(address).await?;
        println!("Server listening on {}", address.to_string());

        loop {
            let (mut stream, _) = listener.accept().await?;
            let routes = self.routes.clone();
            tokio::spawn(async move {
                // This should be a bufreader
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer).await.unwrap();

                let request = parse_request(&buffer).unwrap();
                let future_response = handle_route(&request, &routes).await;

                match future_response {
                    Ok(future) => {
                        let response = future.await;

                        let response_string = format!(
                            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
                            response.status_code,
                            response.status_text,
                            response.headers
                                .iter()
                                .map(|(key, value)| format!("{}: {}", key, value))
                                .collect::<Vec<_>>()
                                .join("\r\n"),
                            response.body.unwrap_or_default()
                        );

                        
                        stream.write(response_string.as_bytes()).await.unwrap();
                        stream.flush().await.unwrap();
                        // Do something with the response
                    }

                    Err(e) => {}
                }
            });
        }
    }
}

fn parse_request(buffer: &[u8]) -> Result<Request, Box<dyn std::error::Error>> {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    let res = match req.parse(&buffer)? {
        httparse::Status::Complete(amt) => amt,
        httparse::Status::Partial => {
            return Err("Request is incomplete".into());
        }
    };

    let method = match req.method.ok_or("Method not found")? {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "DELETE" => HttpMethod::DELETE,
        _ => HttpMethod::OTHER(req.method.unwrap().to_string()),
    };
    let uri = req.path.ok_or("URI not found")?.to_string();
    let version = req.version.ok_or("Version not found")?.to_string();

    let mut headers_map = HashMap::new();
    for header in req.headers.iter() {
        let name = header.name.to_string();
        let value = std::str::from_utf8(header.value)?.to_string();
        headers_map.insert(name, value);
    }

    let body = if res < buffer.len() {
        Some(String::from_utf8(buffer[res..].to_vec())?)
    } else {
        None
    };
    
    
    Ok(Request {
        method,
        uri,
        version,
        headers: headers_map,
        body,
    })
}

async fn handle_route(request: &Request, routes: &HashMap<Route, RouteHandler>) -> FutureResponse {
    // Find the route handler based on the request path and call it
    if let Some(handler) = routes.get(&Route { method: request.method.clone(), path: request.uri.clone() }) {
        handler(request)
    } else {
        
        Err(Box::new(HttpError::InternalServerError(
            HttpStatusCode::InternalServerError,
            "Internal Server Error"
        )))
    }
}

pub struct ServerBuilder {
    address: Option<SocketAddr>,
    routes: Option<HashMap<Route, RouteHandler>>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self {
            address: None,
            routes: Some(HashMap::new()),
        }
    }

    pub fn bind(mut self, socket: SocketAddr) -> Self {
        self.address = Some(socket);
        self
    }

    pub fn route(mut self, path: &str, method:HttpMethod, handler: RouteHandler) -> Self {
        if let Some(routes) = self.routes.as_mut() {
            routes.insert(Route{path:String::from(path),method}, handler);
        } else {
            let mut map = HashMap::new();
            map.insert(Route{path:String::from(path),method}, handler);
            self.routes = Some(map);
        }
        self
    }

    pub fn build(self) -> Result<Server, String> {
        let address = self.address.ok_or("Address not set")?;
        let routes = self.routes.ok_or("Routes Uninitalized")?;
        Ok(Server { address, routes })
    }
}
