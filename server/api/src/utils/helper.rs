use core::time::Duration;
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use moka::future::Cache;
use service::Mutation as MutationCore;
use std::env;
use tokio_cron_scheduler::{Job, JobScheduler};

use super::types::AppState;

pub async fn get_app_state() -> AppState {
    AppState {
        conn: get_db_connection().await.unwrap(),
        cache: Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(1000))
            .build(),
    }
}

pub async fn cron_delete_old_notes() -> anyhow::Result<()> {
    let sched = JobScheduler::new().await?;

    // 0 * * * *

    sched
        .add(Job::new_async("1/7 * * * * *", |_uuid, _l| {
            Box::pin(async {
                let db = get_db_connection().await.unwrap();
                MutationCore::delete_old_notes(&db).await.unwrap();
            })
        })?)
        .await?;

    sched.start().await?;
    Ok(())
}

pub async fn get_db_connection() -> anyhow::Result<DatabaseConnection> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    println!("Connecting to database: {db_url}", db_url = db_url);

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    Ok(conn)
}
