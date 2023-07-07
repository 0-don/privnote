use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Captcha {
    pub text: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetNote {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Note {
    pub note: String,
    pub duration_hours: i32,
    pub manual_password: String,
    pub manual_password_confirm: String,
    pub notify_email: String,
    pub notify_ref: String,
}
