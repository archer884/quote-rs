use std::result;
use model::{Content, SingleQuoteResponse, MultiQuoteResponse};
use serde::{Deserialize, Deserializer};
use service::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct MultiQuotePayload {
    pub quotes: Vec<Quote>
}

impl Content<Quote> for MultiQuoteResponse {
    fn content(self) -> Result<Quote> {
        self.contents.and_then(|mut content| content.quotes.pop()).ok_or(Error::Empty)
    }
}

impl Content<Quote> for SingleQuoteResponse {
    fn content(self) -> Result<Quote> {
        Ok(self.contents.ok_or(Error::Empty)?)
    }
}

#[derive(Debug)]
pub struct Quote {
    pub quote: String,
    pub author: String,
    pub background: Option<String>,
    pub id: String,
}


impl Deserialize for Quote {
    fn deserialize<D: Deserializer>(d: &mut D) -> result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Template {
            pub quote: String,
            pub author: String,
            pub background: Option<String>,
            pub id: String,
        }

        let template = Template::deserialize(d)?;

        Ok(Quote {
            quote: template.quote,
            author: template.author,
            background: template.background,
            id: template.id,
        })
    }
}

#[cfg(test)]
mod tests {
    use model::*;
    use serde_json as json;

    #[test]
    fn deserialize_multi_quote_response() {
        let response = include_str!("../../sample_json/multi-quote.json");
        let quote = json::from_str::<MultiQuoteResponse>(response).expect("unable to deserialize").content().unwrap();
        
        assert_eq!(&quote.quote, "Positive anything is better than negative thinking.");
    }
    
    #[test]
    fn deserialize_single_quote_response() {
        let response = include_str!("../../sample_json/single-quote.json");
        let quote = json::from_str::<SingleQuoteResponse>(response).expect("unable to deserialize").content().unwrap();
        
        assert_eq!(&quote.quote, "Living in the lap of luxury isn't bad, except that you never know when luxury is going to stand up.");
    }

    #[test]
    fn deserialize_failure() {
        let response = include_str!("../../sample_json/unauthorized.json");
        assert!(json::from_str::<MultiQuoteResponse>(response).expect("unable to deserialize").content().is_err());
    }
}
