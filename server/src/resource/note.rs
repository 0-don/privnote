use axum::{http::StatusCode, Json};

use crate::types::types::{GetNote, Note};

pub async fn create_note(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<Note>,
) -> (StatusCode, Json<Note>) {
    // insert your application logic here
    let user = Note {
        note: payload.note,
        ..Default::default()
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

pub async fn get_note(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<GetNote>,
) -> (StatusCode, Json<String>) {
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    print!("{:?}", payload);

    (StatusCode::CREATED, Json("user".to_string()))
}
