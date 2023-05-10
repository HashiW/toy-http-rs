
use serde::{Deserialize, Serialize};

use std::fmt;
#[derive(Debug,Serialize, Deserialize, Clone, Default)]
pub enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
    OTHER(String),
}

#[derive(Debug,Clone, Copy,Default)]
pub enum HttpStatusCode {
    #[default]
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    Conflict = 409,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}



#[derive(Debug,Clone)]
pub enum HttpError {
    BadRequest(HttpStatusCode),
    Unauthorized(HttpStatusCode),
    Forbidden(HttpStatusCode),
    NotFound(HttpStatusCode),
    MethodNotAllowed(HttpStatusCode),
    NotAcceptable(HttpStatusCode),
    Conflict(HttpStatusCode),
    InternalServerError(HttpStatusCode),
    NotImplemented(HttpStatusCode),
    BadGateway(HttpStatusCode),
    ServiceUnavailable(HttpStatusCode),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpError::BadRequest(ErrCode) => write!(f, "Error {}: Bad Request", *ErrCode as u16),
            HttpError::Unauthorized(ErrCode) => write!(f, "Error {}: Unauthorized", *ErrCode as u16),
            HttpError::Forbidden(ErrCode) => write!(f, "Error {}: Forbidden", *ErrCode as u16),
            HttpError::NotFound(ErrCode) => write!(f, "Error {}: Not Found", *ErrCode as u16),
            HttpError::MethodNotAllowed(ErrCode) => write!(f, "Error {}: Method Not Allowed", *ErrCode as u16),
            HttpError::NotAcceptable(ErrCode) => write!(f, "Error {}: Not Acceptable", *ErrCode as u16),
            HttpError::Conflict(ErrCode) => write!(f, "Error {}: Conflict", *ErrCode as u16),
            HttpError::InternalServerError(ErrCode) => write!(f, "Error {}: Internal Server Error", *ErrCode as u16),
            HttpError::NotImplemented(ErrCode) => write!(f, "Error {}: Not Implemented", *ErrCode as u16),
            HttpError::BadGateway(ErrCode) => write!(f, "Error {}: Bad Gateway", *ErrCode as u16),
            HttpError::ServiceUnavailable(ErrCode) => write!(f, "Error {}: Service Unavailable", *ErrCode as u16),
        }
    }
}



impl std::error::Error for HttpError {}