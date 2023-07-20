use argon2::{self, Config};
use chrono::Utc;
use migration::sea_orm::prelude::Uuid;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use service::{
    types::types::{Captcha, DeleteNoteReq, NoteParams},
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
        types::{AppState, CreateNoteResponse, GetNoteResponse},
    },
};

pub async fn create_note(state: State<AppState>, Json(mut create_note): Json<NoteReq>) -> Response {
    let captcha = check_captcha(Captcha::new(&create_note.tag, &create_note.text), &state).await;

    if captcha.is_some() {
        return captcha.unwrap();
    }

    let delete_at =
        (Utc::now() + chrono::Duration::hours(create_note.duration_hours as i64)).naive_utc();

    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    let hash = argon2::hash_encoded(
        create_note.note.as_bytes(),
        salt.as_bytes(),
        &Config::default(),
    )
    .unwrap();

    create_note.note = hash;
    create_note.delete_at = Some(delete_at);

    let note = MutationCore::create_note(&state.conn, create_note)
        .await
        .unwrap();

    Json(ResponseBody::new_data(Some(CreateNoteResponse {
        note,
        secret: salt,
    })))
    .into_response()
}

pub async fn delete_note(
    state: State<AppState>,
    Json(delete_note): Json<DeleteNoteReq>,
) -> Response {
    let captcha = check_captcha(Captcha::new(&delete_note.tag, &delete_note.text), &state).await;

    if captcha.is_some() {
        return captcha.unwrap();
    }

    let is_deleted = MutationCore::delete_note_by_id(&state.conn, delete_note.id)
        .await
        .unwrap();

    Json(ResponseBody::new_data(Some(is_deleted))).into_response()
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
    } else {
        let mut note = note.unwrap();

        let mut deleted = false;
        if note.duration_hours == 0 {
            deleted = MutationCore::delete_note_by_id(&state.conn, note.id)
                .await
                .unwrap();
        }

        let alert = if deleted {
            constants::MESSAGE_NOTE_DELETED.to_string()
        } else {
            note.delete_at.unwrap().to_string()
        };

        aead;

        // note.note = argon2::(&note.note, params.secret.as_bytes()).unwrap();

        return Json(ResponseBody::new_data(Some(GetNoteResponse {
            note,
            alert,
        })))
        .into_response();
    }
}
