use std::io::Read;
use hyper::client;
use serde_json as json;
use serde::de::DeserializeOwned;
use service::Result;

pub trait Response {
    fn model<T: DeserializeOwned>(&mut self) -> Result<T>;
}

impl Response for client::Response {
    fn model<T: DeserializeOwned>(&mut self) -> Result<T> {
        let mut buf = String::new();
        self.read_to_string(&mut buf)?;
        Ok(json::from_str(&buf)?)
    }
}
