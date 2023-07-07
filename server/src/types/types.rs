use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Captcha {
    pub text: String,
    pub hash: String,
}
// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
pub struct GetNote {
    pub id: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}
