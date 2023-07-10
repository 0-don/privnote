mod constants;
mod model;
mod resource;

use crate::{
    model::response::NoSecretResponseBody,
    resource::{
        auth::{get_captcha, verify_captcha},
        note::{create_note, get_note},
    },
};
use axum::{
    http::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use core::{str::FromStr, time::Duration};
use hyper::Server;
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use moka::future::Cache;
use std::{env, net::SocketAddr};

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
    cache: Cache<usize, String>,
}

async fn my_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    // do something with `request`...
    let headers = request.headers();
    let uri = request.uri().clone();
    let secret = headers.get("SECRET").unwrap().to_str().unwrap().to_string();
    println!("{:?}", secret);

    let response = next.run(request).await;

    if uri.to_string().contains("auth") {
        return response;
    }

    Json(vec![NoSecretResponseBody::new(
        constants::MESSAGE_INVALID_TOKEN,
        constants::ERROR_PATH,
    )])
    .into_response()
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
                .route("/note", get(get_note).post(create_note))
                .route_layer(middleware::from_fn(my_middleware)),
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
