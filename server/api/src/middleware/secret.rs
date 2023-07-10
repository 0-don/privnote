use std::env;

use crate::{
    constants,
    model::response::{ResponseBody, ResponseMessages},
};
use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

pub async fn secret_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    let uri = request.uri().clone();

    // run on debug & skip captcha route
    if cfg!(debug_assertions) && uri.to_string().contains("auth") {
        return next.run(request).await;
    }

    let correct_secret = env::var("SECRET").expect("SECRET is not set in .env file");
    let secret = request.headers().get("SECRET");

    if secret.is_none() {
        return Json(ResponseBody::<()>::new_msg(ResponseMessages::new(
            constants::MESSAGE_NO_SECRET.to_owned(),
            constants::ERROR_PATH.to_owned(),
        )))
        .into_response();
    }

    if correct_secret == secret.unwrap().to_str().unwrap() {
        return next.run(request).await;
    }
    
    Json(ResponseBody::<()>::new_msg(ResponseMessages::new(
        constants::MESSAGE_SECRET_NOT_VALID.to_owned(),
        constants::ERROR_PATH.to_owned(),
    )))
    .into_response()
}

//  #[cfg(debug_assertions)]
//     {
//         window.open_devtools();
//     }
