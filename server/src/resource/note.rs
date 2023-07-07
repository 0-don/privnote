use axum::{http::StatusCode, Json};

use crate::types::types::{CreateUser, GetNote, User};

pub async fn create_note(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
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
