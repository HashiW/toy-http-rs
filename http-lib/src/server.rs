use crate::request::Request;
use crate::response::Response;

use crate::http::*;
use std::pin::Pin;
use httparse;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::{fs, io::prelude::*, thread, time::Duration};
use std::future::Future;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Sender};
use std::fmt;




pub type FutureResponse = Result<Pin<Box<dyn Future<Output = Response> + Send>>, Box<dyn std::error::Error + Send>>;

pub type RouteHandler = fn(&Request) -> FutureResponse;

pub struct Server {
    address: SocketAddr,
    routes: HashMap<String, RouteHandler>,
}

impl Server {
    pub async fn run(&self) -> std::io::Result<()> {
        let address = self.address;
        let listener = TcpListener::bind(address).await?;
        println!(
            "Server listening on {}",
            address.to_string()
        );

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

                        let response_string = serde_json::to_string(&response).unwrap();
                
                        stream.write(response_string.as_bytes()).await.unwrap();
                        stream.flush().await.unwrap();
                        // Do something with the response
                    },
                    
                    Err(e) => {
                        
                    }
                }


             
            });
        }
    }
}

fn parse_request(buffer: &[u8]) -> Result<Request, Box<dyn std::error::Error>> {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(&buffer).unwrap();

    let method = match req.method.unwrap() {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "DELETE" => HttpMethod::DELETE,
        _ => HttpMethod::OTHER(req.method.unwrap().to_string()),
    };
    let uri = req.path.unwrap().to_string();
    let version = req.version.unwrap().to_string();

    let mut headers_map = HashMap::new();
    for header in req.headers.iter() {
        let name = header.name.to_string();
        let value = std::str::from_utf8(header.value)?.to_string();
        headers_map.insert(name, value);
    }

    Ok(Request {
        method,
        uri,
        version,
        headers: headers_map,
        body: None,
    })
}

async fn handle_route(request: &Request, routes: &HashMap<String, RouteHandler>) -> FutureResponse {
    // Find the route handler based on the request path and call it
    if let Some(handler) = routes.get(&request.uri) {
        handler(request)
    } else {
        Err(Box::new(HttpError::InternalServerError(HttpStatusCode::InternalServerError)))
    }
}

pub struct ServerBuilder {
    address: Option<SocketAddr>,
    routes: Option<HashMap<String, RouteHandler>>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self {
            address:None,
            routes: Some(HashMap::new()),
        }
    }

    
    pub fn bind(mut self, socket: SocketAddr) -> Self {
        self.address = Some(socket);
        self
    }

    pub fn route(mut self, path: &str, handler: RouteHandler) -> Self {
        if let Some(routes) = self.routes.as_mut() {
            routes.insert(String::from(path), handler);
        } else {
            let mut map = HashMap::new();
            map.insert(String::from(path), handler);
            self.routes = Some(map);
        }
        self
    }

    pub fn mount(mut self, path: &str, routeHandlers: Vec<RouteHandler>) -> Self {
        if let Some(routes) = self.routes.as_mut() {

            for routeHandler in routeHandlers {
                routes.insert(String::from(path), routeHandler);
            }
          
        } else {
            let mut map = HashMap::new();
            for routeHandler in routeHandlers {
                map.insert(String::from(path), routeHandler);
            }
           
           
            self.routes = Some(map);
        }
        self
    }



    pub fn build(self) -> Result<Server, String> {
        let address = self.address.ok_or("Address not set")?;
        let routes = self.routes.ok_or("Routes Uninitalized")?;
        Ok(Server {address, routes })
    }
}