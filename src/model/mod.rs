use serde::Deserialize;

pub mod author;
pub mod category;
pub mod quote;

#[derive(Deserialize)]
pub struct ApiSuccess {
    total: i32,
    range: Option<PageRange>,
}

#[derive(Deserialize)]
pub struct PageRange {
    start: i32,
    end: i32,
}

#[derive(Deserialize)]
pub struct ApiError {
    code: i32,
    message: String,
}

#[derive(Deserialize)]
pub struct ApiResponse<T: Deserialize> {
    success: Option<ApiSuccess>,
    error: Option<ApiError>,
    reason: Option<String>,
    content: Option<T>,
}