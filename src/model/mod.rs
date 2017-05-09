use service::Result;

mod author;
mod category;
mod image;
mod quote;

pub use model::author::*;
pub use model::category::*;
pub use model::image::{Image, ImagePayload};
pub use model::quote::{Quote, MultiQuotePayload};

pub type ImageResponse = ApiResponse<ImagePayload>;
pub type SingleQuoteResponse = ApiResponse<Quote>;
pub type MultiQuoteResponse = ApiResponse<MultiQuotePayload>;

#[derive(Debug, Deserialize)]
pub struct ApiSuccess {
    pub total: i32,
    pub range: Option<PageRange>,
}

#[derive(Debug, Deserialize)]
pub struct PageRange {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub success: Option<ApiSuccess>,
    pub error: Option<ApiError>,
    pub reason: Option<String>,
    pub contents: Option<T>,
}

pub trait Content<T> {
    fn content(self) -> Result<T>;
}
