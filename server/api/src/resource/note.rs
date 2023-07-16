use entity::note;
use migration::sea_orm::prelude::Uuid;
use serde::Deserialize;
use service::{Mutation as MutationCore, Query as QueryCore};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use axum::extract::Path;
use service::types::types::NoteReq;

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

#[derive(Deserialize, Debug)]
pub struct NoteParams {
    pub destroy: Option<u8>,
}

pub async fn get_note(
    state: State<AppState>,
    params: Query<NoteParams>,
    Path(id): Path<Uuid>,
) -> Response {
    let note = QueryCore::find_note_by_id(&state.conn, id).await.unwrap();

    if note.is_none() {
        return Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
            constants::MESSAGE_NOTE_NOT_FOUND.to_string(),
            constants::ERROR_PATH.to_string(),
        )))
        .into_response();
    }

    if params.destroy.is_some() {
        let note = MutationCore::destroy_note_by_id(&state.conn, &note.unwrap())
            .await
            .unwrap();

        return Json(ResponseBody::new_data(Some(note))).into_response();
    }

    return Json(ResponseBody::new_data(note)).into_response();
}
