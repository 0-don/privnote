use entity::note;
use service::{Mutation as MutationCore, Query as QueryCore};

use axum::{extract::State, http::StatusCode, Json};
use service::types::types::{GetNote, NoteReq};

use crate::utils::types::AppState;

pub async fn create_note(
    state: State<AppState>,
    Json(create_note): Json<NoteReq>,
) -> (StatusCode, Json<note::Model>) {
    // println!("{:?}", create_note);

    let note = MutationCore::create_note(&state.conn, create_note)
        .await
        .unwrap();

    (StatusCode::CREATED, Json(note))
}

pub async fn get_note(
    state: State<AppState>,
    Json(payload): Json<GetNote>,
) -> (StatusCode, Json<note::Model>) {
    print!("{:?}", payload);

    let note = QueryCore::find_note_by_id(&state.conn, payload.id)
        .await
        .unwrap()
        .unwrap();

    (StatusCode::CREATED, Json(note))
}
