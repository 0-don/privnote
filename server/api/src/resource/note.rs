use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    Aes256GcmSiv,
    Nonce, // Or `Aes128GcmSiv`
};
use chrono::Utc;
use migration::sea_orm::prelude::Uuid;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use service::{
    types::types::{Captcha, DeleteNoteReq, NoteParams},
    Mutation as MutationCore, Query as QueryCore,
};
use std::str;

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

    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(salt.as_bytes()); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(nonce, create_note.note.as_ref()).unwrap();

    create_note.note = str::from_utf8(&ciphertext.to_owned()).unwrap().to_owned();
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

        let key = Aes256GcmSiv::generate_key(&mut OsRng);
        let cipher = Aes256GcmSiv::new(&key);
        let nonce = Nonce::from_slice(params.secret.as_bytes());
        let plaintext = cipher.decrypt(nonce, note.note.as_ref());

        if plaintext.is_err() {
            return Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
                constants::MESSAGE_NOTE_SECRET_WRONG.to_string(),
                constants::ERROR_PATH.to_string(),
            )))
            .into_response();
        }

        note.note = str::from_utf8(&plaintext.unwrap().to_owned()).unwrap().to_owned();

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

        // note.note = argon2::(&note.note, params.secret.as_bytes()).unwrap();

        return Json(ResponseBody::new_data(Some(GetNoteResponse {
            note,
            alert,
        })))
        .into_response();
    }
}
