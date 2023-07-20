use entity::note;
use migration::sea_orm::DatabaseConnection;
use moka::future::Cache;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub cache: Cache<usize, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNoteResponse {
    pub note: Option<note::Model>,
    pub alert: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteResponse {
    pub note: note::Model,
    pub secret: String,
}
