mod resource;

use axum::{routing::get, Router, Server};
use core::{str::FromStr, time::Duration};
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use moka::future::Cache;
use std::{env, net::SocketAddr};

use crate::resource::{
    auth::{get_captcha, verify_captcha},
    note::{create_note, get_note},
};

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
    cache: Cache<usize, String>,
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    // tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    println!("Connecting to database: {db_url}", db_url = db_url);
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState {
        conn,
        cache: Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(1000))
            .build(),
    };

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/", get(|| async { "Hello, World!" }))
                .route("/auth/captcha", get(get_captcha).post(verify_captcha))
                .route("/note", get(get_note).post(create_note)),
        )
        .with_state(state);

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
