mod resource;
mod types;

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

async fn test() {
    const NUM_TASKS: usize = 16;
    const NUM_KEYS_PER_TASK: usize = 64;

    fn value(n: usize) -> String {
        format!("value {}", n)
    }

    // Create a cache that can store up to 10,000 entries.
    let cache: Cache<usize, String> = Cache::new(10_000);

    // Spawn async tasks and write to and read from the cache.
    let tasks: Vec<_> = (0..NUM_TASKS)
        .map(|i| {
            // To share the same cache across the async tasks, clone it.
            // This is a cheap operation.
            let my_cache = cache.clone();
            let start = i * NUM_KEYS_PER_TASK;
            let end = (i + 1) * NUM_KEYS_PER_TASK;

            tokio::spawn(async move {
                // Insert 64 entries. (NUM_KEYS_PER_TASK = 64)
                for key in start..end {
                    // insert() is an async method, so await it.
                    my_cache.insert(key, value(key)).await;
                    // get() returns Option<String>, a clone of the stored value.
                    assert_eq!(my_cache.get(&key), Some(value(key)));
                }

                // Invalidate every 4 element of the inserted entries.
                for key in (start..end).step_by(4) {
                    // invalidate() is an async method, so await it.
                    my_cache.invalidate(&key).await;
                }
            })
        })
        .collect();

    // Wait for all tasks to complete.
    futures_util::future::join_all(tasks).await;

    // Verify the result.
    for key in 0..(NUM_TASKS * NUM_KEYS_PER_TASK) {
        if key % 4 == 0 {
            assert_eq!(cache.get(&key), None);
        } else {
            assert_eq!(cache.get(&key), Some(value(key)));
        }
    }
}
