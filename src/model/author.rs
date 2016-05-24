#[derive(Deserialize)]
struct Authors {
    authors: Vec<String>
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