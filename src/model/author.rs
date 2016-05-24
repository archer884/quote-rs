use model::{AuthorResponse, Content};
use service::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct AuthorPayload {
    authors: Vec<String>
}

pub type Authors = Vec<String>;

impl Content<Authors> for AuthorResponse {
    fn content(self) -> Result<Authors> {
        self.contents.map(|content| content.authors).ok_or(Error::Empty)
    }
}

#[cfg(test)]
mod tests {
    use model::ApiResponse;
    use model::author::AuthorPayload;
    use serde_json as json;
    
    #[test]
    fn deserialize_success() {
        let response = include_str!("../../sample_json/authors_response.json");
        json::from_str::<ApiResponse<AuthorPayload>>(response).expect("unable to deserialize");
    }
    
    #[test]
    fn deserialize_failure() {
        let response = include_str!("../../sample_json/authors_bad_response.json");
        json::from_str::<ApiResponse<AuthorPayload>>(response).expect("unable to deserialize");
    }
}