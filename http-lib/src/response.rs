use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug,Default, Serialize, Deserialize,Clone)]
pub struct Response {
    pub version: String,
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
