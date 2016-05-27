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

    fn build_uri(&self, method: &str) -> String {
        let mut uri = URI_BASE.to_owned() + "/" + method;
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
        self.client.get(&self.build_uri("qod.json"))
            .send()?
            .model::<QuoteResponse>()?
            .content()
    }

    pub fn qod_categories(&mut self) -> Result<Categories> {
        self.client.get(&self.build_uri("qod/categories.json"))
            .send()?
            .model::<CategoryResponse>()?
            .content()
    }

    pub fn qod_for_category(&mut self, category: &str) -> Result<Quote> {
        self.client.get(&apply_query(
            self.build_uri("qod.json"),
            &Query::new().with_category(category))
        ).send()?.model::<QuoteResponse>()?.content()
    }

    // Everything below here requires an API key

    pub fn random(&mut self) -> Result<Quote> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client.get(&self.build_uri("quote.json"))
            .send()?
            .model::<QuoteResponse>()?
            .content()
    }

    pub fn query(&mut self, query: &Query) -> Result<Quote> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client.get(&apply_query(
            self.build_uri("quote.json"),
            query,
        )).send()?.model::<QuoteResponse>()?.content()
    }

    pub fn categories(&mut self) -> Result<Categories> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client.get(&self.build_uri("quote/categories.json"))
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

        self.client.get(&self.build_uri("quote/image.json"))
            .send()?
            .model::<ImageResponse>()?
            .content()
    }

    pub fn image_query(&mut self, query: &Query) -> Result<Image> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client.get(&apply_query(
            self.build_uri("quote/image.json"),
            query,
        )).send()?.model::<ImageResponse>()?.content()
    }
}

fn apply_query<T: Into<String>>(uri: T, query: &Query) -> String {
    let mut uri = uri.into();
    if !uri.contains('?') {
        uri.push('?');
    }

    query.append_to_buffer(&mut uri);
    uri
}