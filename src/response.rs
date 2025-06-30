use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success { success: bool, data: T },
    Error { success: bool, error: String },
}

pub fn success<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse::Success {
        success: true,
        data,
    }
}

pub fn error<T>(message: &str) -> ApiResponse<T> {
    ApiResponse::Error {
        success: false,
        error: message.to_string(),
    }
}
