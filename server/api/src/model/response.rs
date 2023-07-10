use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessages {
    message: String,
    path: String,
}

impl ResponseMessages {
    pub fn new(message: String, path: String) -> Vec<ResponseMessages> {
        vec![ResponseMessages { message, path }]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub data: Option<T>,
    pub messages: Vec<ResponseMessages>,
}

impl<T> ResponseBody<T> {
    pub fn new_msg(messages: Vec<ResponseMessages>) -> ResponseBody<bool> {
        ResponseBody {
            data: None,
            messages,
        }
    }

    pub fn new_data (data: Option<T>) -> ResponseBody<T> {
        ResponseBody {
            data,
            messages: vec![],
        }
    }
}
