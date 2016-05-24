use std::error::Error;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
struct QuotePayload {
    quotes: Vec<Quote>,
}

pub type Quotes = Vec<Quote>;

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
    use model::ApiResponse;
    use model::quote::QuotePayload;
    use serde_json as json;

    #[test]
    fn deserialize_success() {
        let response = include_str!("../../sample_json/qod.json");
        json::from_str::<ApiResponse<QuotePayload>>(response).expect("unable to deserialize");
    }
    
    #[test]
    fn deserialize_failure() {
        let response = include_str!("../../sample_json/qod_bad_sample.json");
        json::from_str::<ApiResponse<QuotePayload>>(response).expect("unable to deserialize");
    }
}
