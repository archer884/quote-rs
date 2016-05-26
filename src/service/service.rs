use hyper::Client;
use model::*;
use service::response::Response;
use service::{Error, Query, Result};

static URI_BASE: &'static str = "http://quotes.rest";

#[derive(Debug)]
pub struct Service {
    client: Client,
    key: Option<String>,
}

// Struct method implementation
impl Service {
    pub fn new() -> Service {
        Service {
            client: Client::new(),
            key: None,
        }
    }

    pub fn with_key<T: Into<String>>(key: T) -> Service {
        Service {
            client: Client::new(),
            key: Some(key.into()),
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
    pub fn qod(&mut self) -> Result<Quote> {
        self.client.get(&self.build_uri(URI_BASE, "qod.json"))
            .send()?
            .model::<QuoteResponse>()?
            .content()
    }

    pub fn qod_categories(&mut self) -> Result<Categories> {
        self.client.get(&self.build_uri(URI_BASE, "qod/categories.json"))
            .send()?
            .model::<CategoryResponse>()?
            .content()
    }

    pub fn qod_for_category(&mut self, category: &str) -> Result<Quote> {
        self.client.get(&apply_query(
            self.build_uri(URI_BASE, "qod.json"),
            &Query::new().with_category(category))
        ).send()?.model::<QuoteResponse>()?.content()
    }

    // Everything below here requires an API key

    pub fn random(&mut self) -> Result<Quote> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client.get(&self.build_uri(URI_BASE, "quote.json"))
            .send()?
            .model::<QuoteResponse>()?
            .content()
    }

    pub fn query(&mut self, query: &Query) -> Result<Quote> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client.get(&apply_query(
            self.build_uri(URI_BASE, "quote.json"),
            query,
        )).send()?.model::<QuoteResponse>()?.content()
    }

    pub fn categories(&mut self) -> Result<Categories> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client.get(&self.build_uri(URI_BASE, "quote/categories.json"))
            .send()?
            .model::<CategoryResponse>()?
            .content()
    }

    pub fn add(&mut self) -> Result<()> {
        unimplemented!()

        // if self.key.is_none() {
        //     return Err(Error::MethodUnavailable);
        // }
    }

    pub fn image(&mut self) -> Result<Image> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client.get(&self.build_uri(URI_BASE, "quote/image.json"))
            .send()?
            .model::<ImageResponse>()?
            .content()
    }

    pub fn image_query(&mut self, query: &Query) -> Result<Image> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client.get(&apply_query(
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