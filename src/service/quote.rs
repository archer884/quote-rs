use hyper::Client;
use model::*;
use service::response::Response;
use service::{Error, Query, Result};
use std::fmt;

#[cfg(not(feature = "ssl"))]
static URI_BASE: &'static str = "http://quotes.rest";

#[cfg(feature = "ssl")]
static URI_BASE: &'static str = "https://quotes.rest";

pub struct Service {
    client: Client,
    key: Option<String>,
}

// Struct method implementation
impl Service {
    pub fn new() -> Service {
        Service {
            client: create_client(),
            key: None,
        }
    }

    pub fn with_key<T: Into<String>>(key: T) -> Service {
        Service {
            client: create_client(),
            key: Some(key.into()),
        }
    }

    fn build_uri(&self, method: &str) -> String {
        let mut uri = URI_BASE.to_owned() + "/" + method;
        match self.key {
            None => uri,
            Some(ref key) => {
                use std::fmt::Write;
                write!(uri, "?api_key={}", key).unwrap();
                uri
            }
        }
    }
}

impl fmt::Debug for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key({:?})", self.key)
    }
}

#[cfg(feature = "ssl")]
fn create_client() -> Client {
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    
    Client::with_connector(connector)
}

#[cfg(not(feature = "ssl"))]
fn create_client() -> Client {
    Client::new()
}

impl Default for Service {
    fn default() -> Self {
        Service::new()
    }
}

// Service method implementation
impl Service {
    pub fn qod(&self) -> Result<Quote> {
        self.client
            .get(&self.build_uri("qod.json"))
            .send()?
            .model::<MultiQuoteResponse>()?
            .content()
    }

    #[cfg(test)]
    pub fn qod_categories(&self) -> Result<Categories> {
        // These categories are not apt to change, but, if they do,
        // I'm also not apt to know about it, so I'm going to mark
        // this as unsupported.
        Err(Error::MethodNotSupported)
    }

    pub fn qod_for_category(&self, category: &str) -> Result<Quote> {
        self.client
            .get(&apply_query(self.build_uri("qod.json"),
                              &Query::new().by_category(category)))
            .send()?
            .model::<MultiQuoteResponse>()?
            .content()
    }

    // Everything below here requires an API key

    pub fn random(&self) -> Result<Quote> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client
            .get(&self.build_uri("quote.json"))
            .send()?
            .model::<SingleQuoteResponse>()?
            .content()
    }

    pub fn by_id(&self, id: &str) -> Result<Quote> {
        use std::fmt::Write;

        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client
            .get(&{
                let mut uri = self.build_uri("quote.json");
                write!(uri, "&id={}", id).unwrap();
                uri
            })
            .send()?
            .model::<SingleQuoteResponse>()?
            .content()
    }

    pub fn query(&self, query: &Query) -> Result<Quote> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        let uri = apply_query(self.build_uri("quote.json"), query);
        self.client.get(&uri).send()?.model::<SingleQuoteResponse>()?.content()
    }

    pub fn categories(&self, offset: i32) -> Result<Categories> {
        use std::fmt::Write;

        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client
            .get(&{
                let mut uri = self.build_uri("quote/categories.json");
                write!(uri, "&start={}", offset).unwrap();
                uri
            })
            .send()?
            .model::<CategoryResponse>()?
            .content()
    }

    #[cfg(test)]
    pub fn add<T1, T2, T3>(&self, _quote: T1, _author: T2, _tags: &[T3]) -> Result<String>
        where T1: AsRef<str>,
              T2: AsRef<str>,
              T3: AsRef<str>
    {
        // I honestly don't believe this works. The service should return you an ID
        // so that you can view your quote when you submit it, but instead they're
        // just sending back nulls.
        Err(Error::MethodNotSupported)
    }

    pub fn image(&self) -> Result<Image> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client
            .get(&self.build_uri("quote/image.json"))
            .send()?
            .model::<ImageResponse>()?
            .content()
    }

    pub fn image_query(&self, query: &Query) -> Result<Image> {
        if self.key.is_none() {
            return Err(Error::MethodUnavailable);
        }

        self.client
            .get(&apply_query(self.build_uri("quote/image.json"), query))
            .send()?
            .model::<ImageResponse>()?
            .content()
    }
}

fn apply_query<T: Into<String>>(uri: T, query: &Query) -> String {
    let mut uri = uri.into();
    if !uri.contains('?') {
        uri.push('?');
    } else {
        uri.push('&');
    }

    uri.push_str(&query.to_string());
    uri
}
