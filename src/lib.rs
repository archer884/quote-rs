#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(box_syntax, plugin, proc_macro, question_mark)]

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
