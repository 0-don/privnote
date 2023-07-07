use std::env;

use argon2::Config;
use axum::{http::StatusCode, Json};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::types::types::Captcha;

pub async fn get_captcha() -> (StatusCode, Json<Captcha>) {
    let salt = env::var("SALT").expect("SALT not set");

    let text: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let config = Config {
        mem_cost: 256,
        ..Default::default()
    };
    let hash = argon2::hash_encoded(text.as_bytes(), salt.as_bytes(), &config).unwrap();

    (StatusCode::OK, Json(Captcha { hash, text }))
}
