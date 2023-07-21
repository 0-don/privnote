use crate::{
    constants,
    model::response::{ResponseBody, ResponseMessages},
    utils::{
        helper::check_captcha,
        types::{AppState, CreateNoteResponse, GetNoteResponse},
    },
};
use ascon_aead::aead::{Aead, KeyInit};
use ascon_aead::{Ascon128, Key, Nonce}; // Or `Ascon128a`
use axum::extract::Path;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};

use chrono::Utc;
use migration::sea_orm::prelude::Uuid;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use service::types::types::NoteReq;
use service::{
    types::types::{Captcha, DeleteNoteReq},
    Mutation as MutationCore, Query as QueryCore,
};
use std::str;

pub async fn create_note(state: State<AppState>, Json(mut create_note): Json<NoteReq>) -> Response {
    let captcha = check_captcha(Captcha::new(&create_note.tag, &create_note.text), &state).await;

    if captcha.is_some() {
        return captcha.unwrap();
    }

    let delete_at =
        (Utc::now() + chrono::Duration::hours(create_note.duration_hours as i64)).naive_utc();
    create_note.delete_at = Some(delete_at);

    let secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let ciphertext = Ascon128::new(Key::<Ascon128>::from_slice(secret.as_bytes()))
        .encrypt(
            Nonce::<Ascon128>::from_slice(b"unique nonce 012"),
            create_note.note.as_ref(),
        )
        .expect("encryption failure!");

    let note = MutationCore::create_note(&state.conn, create_note, ciphertext)
        .await
        .unwrap();

    Json(ResponseBody::new_data(Some(CreateNoteResponse {
        note,
        secret,
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

    let note = QueryCore::find_note_by_id(&state.conn, Uuid::parse_str(id.unwrap()).unwrap())
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

        let byte_string = Ascon128::new(Key::<Ascon128>::from_slice(secret.as_bytes())).decrypt(
            Nonce::<Ascon128>::from_slice(secret.as_bytes()),
            note.note.as_ref(),
        );

        if byte_string.is_err() {
            return Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
                constants::MESSAGE_NOTE_SECRET_WRONG.to_string(),
                constants::ERROR_PATH.to_string(),
            )))
            .into_response();
        }

        // let bytes = byte_string.unwrap();

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

        let byte_string = byte_string.unwrap();
        return Json(ResponseBody::new_data(Some(GetNoteResponse {
            note,
            text: String::from_utf8_lossy(&*byte_string).to_string(),
            alert,
            // alert,
        })))
        .into_response();
    }
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
