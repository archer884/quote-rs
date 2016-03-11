use std::collections::LinkedList;

use hyper::Client;

use super::{ApiResult, CategoryResponse, Query, QuoteResponse, QuoteError, Quotes};
use super::{read_categories_result, read_quote_result};

pub struct PublicQuoteService {
    quote_log: LinkedList<ApiResult<QuoteResponse>>,
    categories_log: LinkedList<ApiResult<CategoryResponse>>,
}

impl Quotes for PublicQuoteService {
    fn qod(&mut self) -> ApiResult<QuoteResponse> {
        match Client::new().get(
            "http://quotes.rest/qod.json"
        ).send() {
            Err(_) => unimplemented!(), // no idea what to do in this case
            Ok(mut res) => read_quote_result(&mut res),
        }
    }

    fn qod_categories(&mut self) -> ApiResult<CategoryResponse> {
        match Client::new().get(
            "http://quotes.rest/qod/categories.json"
        ).send() {
            Err(_) => unimplemented!(), // no idea what to do in this case
            Ok(mut res) => read_categories_result(&mut res),
        }
    }

    fn qod_for_category(&mut self, category: &str) -> ApiResult<QuoteResponse> {
        match Client::new().get(
            &format!("http://quotes.rest/qod.json?category={}", category)
        ).send() {
            Err(_) => unimplemented!(),
            Ok(mut res) => read_quote_result(&mut res),
        }
    }

    fn random(&mut self) -> ApiResult<QuoteResponse> {
        Err(QuoteError::MethodNotAvailable)
    }

    fn query(&mut self, query: Query) -> ApiResult<QuoteResponse> {
        Err(QuoteError::MethodNotAvailable)
    }

    fn categories(&mut self) -> ApiResult<CategoryResponse> {
        Err(QuoteError::MethodNotAvailable)
    }
}
