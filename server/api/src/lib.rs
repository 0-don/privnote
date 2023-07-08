mod resource;
mod types;

use axum::{routing::get, Router, Server};
use core::str::FromStr;
use std::{env, net::SocketAddr};

use crate::resource::{
    auth::get_captcha,
    note::{create_note, get_note},
};

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let app = Router::new().nest(
        "/api",
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/auth/captcha", get(get_captcha))
            .route("/note", get(get_note).post(create_note)),
    );

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
