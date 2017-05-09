use model::{ApiError, Content, PageRange};
use serde::{Deserialize, Deserializer};
use service::{Error, Result};
use std::result;

#[derive(Debug, Deserialize)]
pub struct AuthorResponse {
    // For whatever stupid-ass reason, the *author* api sends back a *string* for the count, not 
    // a [bleep] number.
    pub success: Option<AuthorSuccess>,
    pub error: Option<ApiError>,
    pub reason: Option<String>,
    pub contents: Option<AuthorPayload>,
}

#[derive(Debug)]
pub struct AuthorSuccess {
    // This may or may not be a number. /sigh
    pub total: Option<i32>,
    pub range: Option<PageRange>,
}

impl<'d> Deserialize<'d> for AuthorSuccess {
    fn deserialize<D: Deserializer<'d>>(d: D) -> result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Template {
            total: String,
            range: Option<PageRange>,
        }

        let template = Template::deserialize(d)?;
        Ok(AuthorSuccess {
            total: template.total.parse().ok(),
            range: template.range,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthorPayload {
    authors: Vec<String>,
}

pub type Authors = Vec<String>;

impl Content<Authors> for AuthorResponse {
    fn content(self) -> Result<Authors> {
        self.contents.map(|content| content.authors).ok_or(Error::Empty)
    }
}

#[cfg(test)]
mod tests {
    use model::AuthorResponse;
    use serde_json as json;

    #[test]
    fn deserialize_success() {
        let response = include_str!("../../sample_json/authors_response.json");
        json::from_str::<AuthorResponse>(response).expect("unable to deserialize");
    }

    #[test]
    fn deserialize_failure() {
        let response = include_str!("../../sample_json/authors_bad_response.json");
        json::from_str::<AuthorResponse>(response).expect("unable to deserialize");
    }
}
