use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponseBody<T> {
    pub message: String,
    pub path: T,
}

impl<T> ErrorResponseBody<T> {
    pub fn new(message: &str, path: T) -> ErrorResponseBody<T> {
        ErrorResponseBody {
            message: message.to_string(),
            path,
        }
    }
}
