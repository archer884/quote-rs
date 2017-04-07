#![feature(box_syntax, plugin, proc_macro)]

#[cfg(feature = "ssl")]
extern crate hyper_native_tls;

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

mod model;
mod service;

pub use service::{Error, Query, Result, Service};
pub use model::{Authors, Categories, Image, Quote};
