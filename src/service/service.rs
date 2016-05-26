use hyper::Client;
use model::*;
use service::response::Response;
use service::{Error, Query, Result};

static URI_BASE: &'static str = "http://quotes.rest";

#[derive(Debug)]
pub struct Service {
    key: Option<String>
}

// Struct method implementation
impl Service {
    pub fn new() -> Service {
        Service { key: None }
    }

    pub fn with_key<T: Into<String>>(key: T) -> Service {
        Service {
            key: Some(key.into())
        }
    }

    fn build_uri(&self, base: &str, method: &str) -> String {
        let mut uri = base.to_owned() + "/" + method;
        match self.key {
            None => uri,
            Some(ref key) => {
                uri.push_str(&format!("api_key={}", key));
                uri
            }
        }
    }
}

impl Default for Service {
    fn default() -> Self {
        Service::new()
    }
}

// Service method implementation
impl Service {
    // GET: http://quotes.rest/qod.json
    pub fn qod(&mut self) -> Result<Quote> {
        Client::new().get(&self.build_uri(URI_BASE, "qod.json"))
            .send()?
            .model::<QuoteResponse>()?
            .content()
    }

    // GET: http://quotes.rest/qod/categories.json
    pub fn qod_categories(&mut self) -> Result<Categories> {
        Client::new().get(&self.build_uri(URI_BASE, "qod/categories.json"))
            .send()?
            .model::<CategoryResponse>()?
            .content()
    }

    // GET: http://quotes.rest/qod/qod.json?category=testing
    pub fn qod_for_category(&mut self, category: &str) -> Result<Quote> {
        Client::new().get(&apply_query(
            self.build_uri(URI_BASE, "qod.json"),
            &Query::new().with_category(category))
        ).send()?.model::<QuoteResponse>()?.content()
    }

    // Everything below here requires an API key

    // GET: http://quotes.rest/quote.json
    pub fn random(&mut self) -> Result<Quote> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        Client::new().get(&self.build_uri(URI_BASE, "quote.json"))
            .send()?
            .model::<QuoteResponse>()?
            .content()
    }

    // GET: http://quotes.rest/quote.json?minlength=100&maxlength=300
    pub fn query(&mut self, query: &Query) -> Result<Quote> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        Client::new().get(&apply_query(
            self.build_uri(URI_BASE, "quote.json"),
            query,
        )).send()?.model::<QuoteResponse>()?.content()
    }

    // GET: http://quotes.rest/quote/categories.json
    pub fn categories(&mut self) -> Result<Categories> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        Client::new().get(&self.build_uri(URI_BASE, "quote/categories.json"))
            .send()?
            .model::<CategoryResponse>()?
            .content()
    }

    // PUT: http://quotes.rest/quote.json?quote=<quote>&author=<author>&tags=<tags>
    pub fn add(&mut self) -> Result<()> {
        unimplemented!()

        // if self.key.is_none() {
        //     return Err(Error::MethodUnavailable);
        // }
    }

    // GET: http://quotes.rest/quote/image.json
    pub fn image(&mut self) -> Result<Image> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        Client::new().get(&self.build_uri(URI_BASE, "quote/image.json"))
            .send()?
            .model::<ImageResponse>()?
            .content()
    }

    // GET: http://quotes.rest/quote/image.json?author=<author>&category=<category>
    pub fn image_query(&mut self, query: &Query) -> Result<Image> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        Client::new().get(&apply_query(
            self.build_uri(URI_BASE, "quote/image.json"),
            query,
        )).send()?.model::<ImageResponse>()?.content()
    }
}

fn apply_query<T: Into<String>>(uri: T, query: &Query) -> String {
    let mut uri = uri.into();
    let query = query.to_string();
    uri.reserve_exact(query.len() + 1);

    if !uri.contains('?') {
        uri.push('?');
    }

    uri.push_str(&query.to_string());
    uri
}