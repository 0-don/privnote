use axum::{extract::State, http::StatusCode, Json};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use service::types::types::CsrfToken;

use crate::utils::types::AppState;

pub async fn get_csrf_token(state: State<AppState>) -> (StatusCode, Json<CsrfToken>) {
    let id = rand::thread_rng().gen_range(0..255);
    let text: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(id)
        .map(char::from)
        .collect();

    state.cache.insert(id, text.clone()).await;

    (StatusCode::OK, Json(CsrfToken { tag: id, text }))
}
