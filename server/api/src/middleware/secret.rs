use crate::{constants, model::response::NoSecretResponseBody};
use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

pub async fn secret_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    let headers = request.headers();
    let uri = request.uri().clone();
    let secret = headers.get("SECRET").unwrap().to_str().unwrap().to_string();
    println!("{:?}", secret);

    if uri.to_string().contains("auth") {
        return next.run(request).await;
    }

    Json(vec![NoSecretResponseBody::new(
        constants::MESSAGE_NO_SECRET,
        constants::ERROR_PATH,
    )])
    .into_response()
}
