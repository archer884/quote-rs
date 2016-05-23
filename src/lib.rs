#![feature(custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate regex;
extern crate serde;
extern crate serde_json;

use std::io::Read;
use std::fmt;
use hyper::Client;
use hyper::client::response::Response as HyperResponse;
use hyper::header::{Authorization, Basic, ContentType};
use regex::Regex;

mod model;

use model::quote::QuoteResponse;

// mod public;
// mod quote;

// pub use public::PublicQuoteService;
// pub use quote::Quote;

pub type ApiResult<T> = Result<T, QuoteError>;

pub struct CategoryResponse;

pub enum QuoteError {
    MethodNotAvailable,     // not all methods are available for the public quote service
}

// API Secret: X-TheySaidSo-Api-Secret
trait Quotes {
    // GET: http://quotes.rest/qod.json
    fn qod(&mut self) -> ApiResult<QuoteResponse>;

    // GET: http://quotes.rest/qod/categories.json
    fn qod_categories(&mut self) -> ApiResult<CategoryResponse>;

    // GET: http://quotes.rest/qod/qod.json?category=testing
    fn qod_for_category(&mut self, category: &str) -> ApiResult<QuoteResponse>;

    // GET: http://quotes.rest/quote.json
    fn random(&mut self) -> ApiResult<QuoteResponse>;

    // GET: http://quotes.rest/quote.json?minlength=100&maxlength=300
    fn query(&mut self, Query) -> ApiResult<QuoteResponse>;

    // GET: http://quotes.rest/quote/categories.json
    fn categories(&mut self) -> ApiResult<CategoryResponse>;
}

pub struct Query {
    min: Option<i32>,
    max: Option<i32>,
    category: Option<String>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            min: None,
            max: None,
            category: None,
        }
    }

    pub fn with_min(mut self, min: i32) -> Query {
        self.min = Some(min);
        self
    }

    pub fn with_max(mut self, max: i32) -> Query {
        self.max = Some(max);
        self
    }

    pub fn with_category<T: Into<String>>(mut self, category: T) -> Query {
        self.category = Some(category.into());
        self
    }
}

fn read_quote_result(res: &mut HyperResponse) -> ApiResult<QuoteResponse> {
    let mut buf = String::new();
    res.read_to_string(&mut buf);

    unimplemented!()
}

fn read_categories_result(res: &mut HyperResponse) -> ApiResult<CategoryResponse> {
    unimplemented!()
}

fn build_query(query: &Query) -> String {
    let mut parameters = Vec::new();

    if let Some(min) = query.min { parameters.push(format!("minlength={}", min)); }
    if let Some(max) = query.max { parameters.push(format!("maxlength={}", max)); }
    if let Some(ref category) = query.category { parameters.push(format!("category={}", category)); }

    let query = "http://quotes.rest/quote.json".to_owned();
    if parameters.len() > 0 {
        query + "?" + &parameters.join("&")
    } else {
        query
    }
}

// #[cfg(test)]
// mod tests {
//     use super::{build_query, Query, QuoteResponse};
//     use serde_json as json;

//     #[test]
//     fn build_query_produces_expected_results_for_min_and_max() {
//         let query = Query::new().with_min(20);
//         assert_eq!(
//             "http://quotes.rest/quote.json?minlength=20",
//             &build_query(&query)
//         );

//         let query = Query::new().with_max(20);
//         assert_eq!(
//             "http://quotes.rest/quote.json?maxlength=20",
//             &build_query(&query)
//         );

//         let query = Query::new().with_min(20).with_max(20);
//         assert_eq!(
//             "http://quotes.rest/quote.json?minlength=20&maxlength=20",
//             &build_query(&query)
//         );
//     }

//     #[test]
//     fn build_query_produces_expected_results_for_category() {
//         let query = Query::new().with_category("testing");
//         assert_eq!(
//             "http://quotes.rest/quote.json?category=testing",
//             &build_query(&query)
//         );

//         let query = Query::new().with_category("testing").with_min(20).with_max(20);
//         assert_eq!(
//             "http://quotes.rest/quote.json?minlength=20&maxlength=20&category=testing",
//             &build_query(&query)
//         );
//     }
// }
