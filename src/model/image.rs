use model::{Content, ImageResponse};
use service::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct ImagePayload {
    qimage: Image,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub id: String,
    pub quote_id: String,
    pub permalink: String,
    pub download_uri: String,
}

impl Content<Image> for ImageResponse {
    fn content(self) -> Result<Image> {
        self.contents.map(|content| content.qimage).ok_or(Error::Empty)
    }
}

#[cfg(test)]
mod tests {
    use model::ApiResponse;
    use model::image::ImagePayload;
    use serde_json as json;

    #[test]
    fn deserialize_success() {
        let response = include_str!("../../sample_json/image.json");
        json::from_str::<ApiResponse<ImagePayload>>(response).expect("unable to deserialize");
    }

    #[test]
    fn deserialize_failure() {
        let response = include_str!("../../sample_json/bad-category.json");
        json::from_str::<ApiResponse<ImagePayload>>(response).expect("unable to deserialize");
    }
}
