use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NoSecretResponseBody<T> {
    pub message: String,
    pub path: T,
}

impl<T> NoSecretResponseBody<T> {
    pub fn new(message: &str, path: T) -> NoSecretResponseBody<T> {
        NoSecretResponseBody {
            message: message.to_string(),
            path,
        }
    }
}
