use axum::{http::StatusCode, Json};
use entity::note::ActiveModel;
use migration::sea_orm::Set;
use service::types::types::{GetNote, NoteReq};

pub async fn create_note(Json(note): Json<NoteReq>) -> (StatusCode, Json<NoteReq>) {
    println!("{:?}", note);

    (StatusCode::CREATED, Json(note))
}

pub async fn get_note(Json(payload): Json<GetNote>) -> (StatusCode, Json<String>) {
    print!("{:?}", payload);

    (StatusCode::CREATED, Json("user".to_string()))
}
