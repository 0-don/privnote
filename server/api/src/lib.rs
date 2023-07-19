mod constants;
mod middleware;
mod model;
mod resource;
mod utils;

use crate::{
    middleware::secret::secret_middleware,
    resource::{
        auth::get_captcha,
        note::{create_note, delete_note, get_note},
    },
    utils::helper::{cron_delete_old_notes, get_app_state},
};
use axum::{
    middleware::{self as Middleware},
    routing::{get, post},
    Router,
};
use core::str::FromStr;
use hyper::Server;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    // tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    cron_delete_old_notes().await?;

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/", get(|| async { "Hello, World!" }))
                .route("/auth/captcha", get(get_captcha))
                .route("/note/:id", get(get_note))
                .route("/note", post(create_note).delete(delete_note))
                .route_layer(Middleware::from_fn(secret_middleware)),
        )
        .with_state(get_app_state().await);

    println!("http://{}/api", server_url);

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
