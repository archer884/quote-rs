use model::{CategoryResponse, Content};
use service::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct CategoryPayload {
    pub categories: Vec<String>
}

pub type Categories = Vec<String>;

impl Content<Categories> for CategoryResponse {
    fn content(self) -> Result<Categories> {
        self.contents.map(|content| content.categories).ok_or(Error::Empty)
    }
}

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