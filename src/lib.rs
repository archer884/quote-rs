#![feature(box_syntax, custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde_json;
extern crate serde;
extern crate url;

mod model;
mod service;

pub use service::{
    Error,
    Query,
    Result,
    Service,
};

pub use model::{
    Authors,
    Categories,
    Image,
    Quote,
};