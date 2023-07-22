use crate::{
    constants,
    model::response::{ResponseBody, ResponseMessages},
    utils::{
        helper::check_csrf_token,
        types::{AppState, CreateNoteResponse, GetNoteResponse},
    },
};
use axum::extract::Path;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use service::types::types::NoteReq;
use service::{
    types::types::{CsrfToken, DeleteNoteReq},
    Mutation as MutationCore, Query as QueryCore,
};
use std::str;

pub async fn create_note(state: State<AppState>, Json(mut create_note): Json<NoteReq>) -> Response {
    let csrf = check_csrf_token(CsrfToken::new(&create_note.tag, &create_note.text), &state).await;

    if csrf.is_some() {
        return csrf.unwrap();
    }

    let delete_at =
        (Utc::now() + chrono::Duration::hours(create_note.duration_hours as i64)).naive_utc();
    create_note.delete_at = Some(delete_at);

    let og_secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let mut secret = og_secret.clone();

    if !create_note.manual_password.is_empty() {
        secret = format!("{}{}", secret, create_note.manual_password);
        create_note.manual_password =
            new_magic_crypt!(&secret, 256).encrypt_bytes_to_base64(&create_note.manual_password)
    }

    if !create_note.notify_email.is_empty() {
        create_note.notify_email =
            new_magic_crypt!(&secret, 256).encrypt_bytes_to_base64(&create_note.notify_email)
    }

    create_note.note = new_magic_crypt!(&secret, 256).encrypt_bytes_to_base64(&create_note.note);

    let note = MutationCore::create_note(&state.conn, create_note)
        .await
        .unwrap();

    Json(ResponseBody::new_data(Some(CreateNoteResponse {
        note,
        secret: og_secret,
    })))
    .into_response()
}

pub async fn get_note(state: State<AppState>, Path(id): Path<String>) -> Response {
    let list = id.split("@").collect::<Vec<&str>>();

    let id = list.get(0);
    let secret = list.get(1);

    if id.is_none() || secret.is_none() {
        return Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
            constants::MESSAGE_NO_ID_NO_SECRET.to_string(),
            constants::ERROR_PATH.to_string(),
        )))
        .into_response();
    }

    let note = QueryCore::find_note_by_id(&state.conn, id.unwrap().to_string())
        .await
        .unwrap();

    if note.is_none() {
        return Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
            constants::MESSAGE_NOTE_NOT_FOUND.to_string(),
            constants::ERROR_PATH.to_string(),
        )))
        .into_response();
    } else {
        let note = note.unwrap();
        let secret = secret.unwrap();

        let text = new_magic_crypt!(&secret, 256).decrypt_base64_to_string(&note.note);

        if text.is_err() {
            return Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
                constants::MESSAGE_NOTE_SECRET_WRONG.to_string(),
                constants::ERROR_PATH.to_string(),
            )))
            .into_response();
        }

        let mut deleted = false;
        if note.duration_hours == 0 {
            deleted = MutationCore::delete_note_by_id(&state.conn, &note.id)
                .await
                .unwrap();
        }

        let alert = if deleted {
            constants::MESSAGE_NOTE_DELETED.to_string()
        } else {
            note.delete_at.to_string()
        };

        return Json(ResponseBody::new_data(Some(GetNoteResponse {
            note,
            text: text.unwrap(),
            alert,
        })))
        .into_response();
    }
}

pub async fn delete_note(
    state: State<AppState>,
    Json(delete_note): Json<DeleteNoteReq>,
) -> Response {
    let captcha =
        check_csrf_token(CsrfToken::new(&delete_note.tag, &delete_note.text), &state).await;

    if captcha.is_some() {
        return captcha.unwrap();
    }

    let is_deleted = MutationCore::delete_note_by_id(&state.conn, &delete_note.id)
        .await
        .unwrap();

    Json(ResponseBody::new_data(Some(is_deleted))).into_response()
}
