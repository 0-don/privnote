use axum::{extract::State, http::StatusCode, Json};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::{types::types::Captcha, AppState};

pub async fn get_captcha(state: State<AppState>) -> (StatusCode, Json<Captcha>) {
    let id = rand::thread_rng().gen_range(0..255);
    let text: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(id)
        .map(char::from)
        .collect();

    state.cache.insert(id, text.clone()).await;

    (StatusCode::OK, Json(Captcha { id, text }))
}

pub async fn verify_captcha(
    state: State<AppState>,
    Json(captcha): Json<Captcha>,
) -> (StatusCode, Json<Captcha>) {
    let rest = state.cache.get(&captcha.id);

    if rest.is_none() || rest.unwrap() != captcha.text {
        println!("Captcha verification failed: {:?}", captcha);
        return (StatusCode::BAD_REQUEST, Json(Captcha::default()));
    }

    println!("Captcha verification success: {:?}", captcha);
    (StatusCode::OK, Json(captcha))
}
