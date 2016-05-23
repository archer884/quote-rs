use std::error::Error;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Quote {
    pub quote: String,
    pub length: i32,
    pub author: String,
    pub tags: Vec<String>,
    pub category: String,
    pub date: String,
    pub title: String,
    pub background: String,
    pub id: String,
}

pub type Quotes = Vec<Quote>;
pub type Reason = String;

pub enum QuoteResponse {
    Success(Quotes),
    Failure(Reason),
}

impl QuoteResponse {
    pub fn success(quotes: Quotes) -> QuoteResponse {
        QuoteResponse::Success(quotes)
    }
    
    pub fn failure<T: Into<String>>(reason: T) -> QuoteResponse {
        QuoteResponse::Failure(reason.into())
    } 
    
    pub fn quotes(self) -> Option<Quotes> {
        match self {
            QuoteResponse::Success(quotes) => Some(quotes),
            QuoteResponse::Failure(_) => None,
        }
    }
    
    pub fn reason(self) -> Option<String> {
        match self {
            QuoteResponse::Failure(reason) => Some(reason),
            QuoteResponse::Success(_) => None,
        }
    }
}

impl Deserialize for QuoteResponse {
    fn deserialize<D: Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Template {
            contents: Option<Contents>,
            reason: Option<String>,
        }
        
        #[derive(Deserialize)]
        struct Contents {
            quotes: Vec<Quote>
        }
        
        let template = Template::deserialize(d)?;
        if let Some(contents) = template.contents {
            return Ok(QuoteResponse::Success(contents.quotes));
        }
        
        if let Some(reason) = template.reason {
            return Ok(QuoteResponse::Failure(reason));
        }
        
        return Err(SerdeError::custom("Unknown error"));
    }
}

impl Deserialize for Quote {
    fn deserialize<D: Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Template {
            pub quote: String,
            pub length: String,
            pub author: String,
            pub tags: Vec<String>,
            pub category: String,
            pub date: String,
            pub title: String,
            pub background: String,
            pub id: String,
        }
        
        let template = Template::deserialize(d)?;
        
        Ok(Quote {
            quote: template.quote,
            length: template.length.parse::<i32>().map_err(|e| SerdeError::custom(e.description()))?,
            author: template.author,
            tags: template.tags,
            category: template.category,
            date: template.date,
            title: template.title,
            background: template.background,
            id: template.id,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::{Quote, QuoteResponse};
    use serde_json as json;
    use serde::de::Error as SerdeError;

    #[test]
    fn deserialize_success() {
        let response = include_str!("../../sample_json/qod.json");
        let target = Quote {
            quote: "Positive anything is better than negative thinking.".to_owned(),
            length: 51,
            author: "Elbert Hubbard".to_owned(),
            tags: vec!["inspire".to_owned(), "positive-thinking".to_owned()],
            category: "inspire".to_owned(),
            date: "2016-01-16".to_owned(),
            title: "Inspiring Quote of the day".to_owned(),
            background: "https://theysaidso.com/img/bgs/man_on_the_mountain.jpg".to_owned(),
            id: "g3j5nQxVRTka7Sw3khgdRQeF".to_owned(),
        };

        let quote_response = json::from_str::<QuoteResponse>(response).expect("unable to deserialize");
        let quotes = quote_response.quotes().expect("no quotes");
        let quote = quotes.first().expect("empty quote vec");

        assert_eq!(target.quote, quote.quote);
        assert_eq!(target.length, quote.length);
        assert_eq!(target.author, quote.author);
        assert_eq!(target.tags, quote.tags);
        assert_eq!(target.category, quote.category);
        assert_eq!(target.date, quote.date);
        assert_eq!(target.title, quote.title);
        assert_eq!(target.background, quote.background);
        assert_eq!(target.id, quote.id);
    }
    
    #[test]
    fn deserialize_failure() {
        let response = include_str!("../../sample_json/qod_bad_sample.json");
        let target = QuoteResponse::failure("QOD Category not supported");
        
        let quote_response = json::from_str::<QuoteResponse>(response).expect("unable to deserialize");
        
        assert_eq!(target.reason(), quote_response.reason());
    }
}
