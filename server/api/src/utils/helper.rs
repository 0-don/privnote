use core::time::Duration;
use migration::{sea_orm::Database, Migrator, MigratorTrait};
use moka::future::Cache;
use std::env;

use super::types::AppState;

pub async fn get_app_state() -> AppState {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    println!("Connecting to database: {db_url}", db_url = db_url);

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    AppState {
        conn,
        cache: Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(1000))
            .build(),
    }
}
