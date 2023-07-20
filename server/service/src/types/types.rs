use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Captcha {
    pub tag: usize,
    pub text: String,
}

impl Captcha {
    pub fn new(tag: &usize, text: &String) -> Captcha {
        Captcha {
            tag: tag.clone(),
            text: text.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct NoteParams {
    pub destroy: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetNoteReq {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeleteNoteReq {
    pub tag: usize,
    pub text: String,

    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NoteReq {
    pub tag: usize,
    pub text: String,

    pub note: String,
    pub duration_hours: i32,
    pub manual_password: String,
    pub manual_password_confirm: String,
    pub notify_email: String,
    pub notify_ref: String,

    pub delete_at: Option<chrono::NaiveDateTime>,
}
