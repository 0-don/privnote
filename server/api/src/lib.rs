mod resource;
mod types;

use std::env;

use axum::{routing::get, Router};

use crate::resource::{
    auth::get_captcha,
    note::{create_note, get_note},
};

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Failed to read .env file");

    tracing_subscriber::fmt::init();

    let server_port = env::var("SERVER_PORT").expect("PORT not set");

    let app = Router::new().nest(
        "/api",
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/auth/captcha", get(get_captcha))
            .route("/note", get(get_note).post(create_note)),
    );

    println!("http://127.0.0.1:{}/api", server_port);

    axum::Server::bind(&format!("0.0.0.0:{}", server_port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
