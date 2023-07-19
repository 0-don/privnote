use migration::sea_orm::prelude::Uuid;
use service::{
    types::types::{Captcha, NoteParams, DeleteNoteReq},
    Mutation as MutationCore, Query as QueryCore,
};

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Json,
};

use axum::extract::Path;
use service::types::types::NoteReq;

use crate::{
    constants,
    model::response::{ResponseBody, ResponseMessages},
    utils::{
        helper::check_captcha,
        types::{AppState, NoteResponse},
    },
};

pub async fn create_note(state: State<AppState>, Json(create_note): Json<NoteReq>) -> Response {
    let captcha = check_captcha(Captcha::new(&create_note.tag, &create_note.text), &state).await;

    if captcha.is_some() {
        return captcha.unwrap();
    }

    let note = MutationCore::create_note(&state.conn, create_note)
        .await
        .unwrap();

    Json(ResponseBody::new_data(Some(note))).into_response()
}

pub async fn delete_note(state: State<AppState>, Json(create_note): Json<DeleteNoteReq>) -> Response {
    let captcha = check_captcha(Captcha::new(&create_note.tag, &create_note.text), &state).await;

    if captcha.is_some() {
        return captcha.unwrap();
    }

    let note = MutationCore::delete_note_by_id(&state.conn, create_note.id)
        .await
        .unwrap();

    Json(ResponseBody::new_data(Some(note))).into_response()
}

pub async fn get_note(
    state: State<AppState>,
    _params: Query<NoteParams>,
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

    let mut deleted = false;
    if note.as_ref().unwrap().duration_hours == 0 {
        deleted = MutationCore::delete_note_by_id(&state.conn, note.as_ref().unwrap().id)
            .await
            .unwrap();
    }

    let alert = if deleted {
        constants::MESSAGE_NOTE_DELETED.to_string()
    } else {
        note.as_ref().unwrap().delete_at.unwrap().to_string()
    };

    return Json(ResponseBody::new_data(Some(NoteResponse { note, alert }))).into_response();
}
