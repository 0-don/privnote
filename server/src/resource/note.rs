use axum::{http::StatusCode, Json};

use crate::types::types::{GetNote, Note};

pub async fn create_note(Json(payload): Json<Note>) -> (StatusCode, Json<Note>) {
    let note = Note {
        note: payload.note,
        ..Default::default()
    };

    // println!("{:?}", note);

    (StatusCode::CREATED, Json(note))
}

pub async fn get_note(Json(payload): Json<GetNote>) -> (StatusCode, Json<String>) {
    print!("{:?}", payload);

    (StatusCode::CREATED, Json("user".to_string()))
}
