use entity::note;
use service::{Mutation as MutationCore, Query as QueryCore};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use service::types::types::{GetNote, NoteReq};

use crate::{
    constants,
    model::response::{ResponseBody, ResponseMessages},
    utils::types::AppState,
};

pub async fn create_note(state: State<AppState>, Json(create_note): Json<NoteReq>) -> Response {
    let text = state.cache.get(&create_note.tag);
    if text.is_none() {
        return Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
            constants::MESSAGE_NO_TAG_OR_NO_TEXT.to_string(),
            constants::ERROR_PATH.to_string(),
        )))
        .into_response();
    }

    if create_note.text != text.unwrap() {
        return Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
            constants::MESSAGE_CAPTCHA_WRONG.to_string(),
            constants::ERROR_PATH.to_string(),
        )))
        .into_response();
    }

    let note = MutationCore::create_note(&state.conn, create_note)
        .await
        .unwrap();

    Json(ResponseBody::new_data(Some(note))).into_response()
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
