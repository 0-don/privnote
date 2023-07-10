use migration::sea_orm::DatabaseConnection;
use moka::future::Cache;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub cache: Cache<usize, String>,
}
