use model::{ApiError, Content, PageRange};
use serde::{Deserialize, Deserializer};
use service::{Error, Result};
use std::result;

#[derive(Debug, Deserialize)]
pub struct CategoryResponse {
    // For whatever stupid-ass reason, the *author* api sends back a *string* for the count, not 
    // a [bleep] number.
    pub success: Option<CategorySuccess>,
    pub error: Option<ApiError>,
    pub reason: Option<String>,
    pub contents: Option<CategoryPayload>,
}

#[derive(Debug)]
pub struct CategorySuccess {
    // This may or may not be a number. /sigh
    pub total: Option<i32>,
    pub range: Option<PageRange>,
}

impl<'d> Deserialize<'d> for CategorySuccess {
    fn deserialize<D: Deserializer<'d>>(d: D) -> result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Template {
            total: String,
            range: Option<PageRange>,
        }

        let template = Template::deserialize(d)?;
        Ok(CategorySuccess {
            total: template.total.parse().ok(),
            range: template.range,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CategoryPayload {
    pub categories: Vec<String>,
}

pub type Categories = Vec<String>;

impl Content<Categories> for CategoryResponse {
    fn content(self) -> Result<Categories> {
        self.contents.map(|content| content.categories).ok_or(Error::Empty)
    }
}

#[cfg(test)]
mod tests {
    use model::CategoryResponse;
    use serde_json as json;

    #[test]
    fn deserialize_categories() {
        let response = include_str!("../../sample_json/category.json");
        json::from_str::<CategoryResponse>(response).expect("unable to deserialize");
    }

    #[test]
    fn deserialize_failure() {
        let response = include_str!("../../sample_json/bad-category.json");
        json::from_str::<CategoryResponse>(response).expect("unable to deserialize");
    }
}
