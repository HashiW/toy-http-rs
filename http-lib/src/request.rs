
use std::collections::HashMap;

use crate::http::HttpMethod;


use serde::{Deserialize, Serialize};
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}