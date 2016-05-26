use std::io::Read;
use hyper::client;
use serde_json as json;
use serde::Deserialize;
use service::Result;

pub trait Response {
    fn model<T: Deserialize>(&mut self) -> Result<T>;
}

impl Response for client::Response {
    fn model<T: Deserialize>(&mut self) -> Result<T> {
        let mut buf = String::new();
        self.read_to_string(&mut buf)?;
        Ok(json::from_str(&buf)?)
    }
}