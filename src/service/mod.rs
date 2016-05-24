use std::error;
use std::fmt;
use std::io;
use hyper;
use serde_json as json;

mod query;
mod response;
mod service;

pub use service::query::Query;
pub use service::service::Service;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Empty,
    IO(Box<error::Error + Send>),
    Json(Box<error::Error + Send>),
    MethodUnavailable,
    NetworkError(Box<error::Error + Send>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Empty => write!(f, "Empty"),
            Error::IO(ref e) => write!(f, "IO error: {}", e),
            Error::Json(ref e) => write!(f, "Json error: {}", e),
            Error::MethodUnavailable => write!(f, "Method unavailable"),
            Error::NetworkError(ref e) => write!(f, "Network error: {}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Empty => "Empty response",
            Error::IO(_) => "IO failure",
            Error::Json(_) => "Json failure",
            Error::MethodUnavailable => "Method unavailable",
            Error::NetworkError(_) => "Network failure"
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Error {
        Error::NetworkError(box error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO(box error)
    }
}

impl From<json::Error> for Error {
    fn from(error: json::Error) -> Error {
        Error::Json(box error)
    }
}