#[derive(Deserialize)]
struct CategoryPayload {
    categories: Vec<String>
}

pub type Categories = Vec<String>;

#[cfg(test)]
mod tests {
    use model::ApiResponse;
    use model::category::CategoryPayload;
    use serde_json as json;
    
    #[test]
    fn deserialize_success() {
        let response = include_str!("../../sample_json/categories_response.json");
        json::from_str::<ApiResponse<CategoryPayload>>(response).expect("unable to deserialize");
    }
    
    #[test]
    fn deserialize_failure() {
        let response = include_str!("../../sample_json/categories_bad_response.json");
        json::from_str::<ApiResponse<CategoryPayload>>(response).expect("unable to deserialize");
    }
}