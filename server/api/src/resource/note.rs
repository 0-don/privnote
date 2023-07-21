use crate::{
    constants,
    model::response::{ResponseBody, ResponseMessages},
    utils::{
        helper::check_captcha,
        types::{AppState, CreateNoteResponse, GetNoteResponse},
    },
};
use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    Aes256GcmSiv, Nonce,
};
use axum::extract::Path;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use migration::sea_orm::prelude::Uuid;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use service::types::types::NoteReq;
use service::{
    types::types::{Captcha, DeleteNoteReq, NoteParams},
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

    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    let ciphertext = Aes256GcmSiv::new(&Aes256GcmSiv::generate_key(&mut OsRng))
        .encrypt(
            Nonce::from_slice(salt.as_bytes()),
            create_note.note.as_ref(),
        )
        .unwrap();

    println!("{:?}", String::from_utf8_lossy(&*ciphertext));

    create_note.delete_at = Some(delete_at);

    let note = MutationCore::create_note(&state.conn, create_note, ciphertext)
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
        let mut note = note.unwrap();

        let plaintext = Aes256GcmSiv::new(&Aes256GcmSiv::generate_key(&mut OsRng)).decrypt(
            Nonce::from_slice(secret.unwrap().as_bytes()),
            note.note.as_ref(),
        );

        if plaintext.is_err() {
            return Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
                constants::MESSAGE_NOTE_SECRET_WRONG.to_string(),
                constants::ERROR_PATH.to_string(),
            )))
            .into_response();
        }

        let text = str::from_utf8(&plaintext.unwrap()).unwrap().to_string();
        println!("{}", text);

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
            text,
            alert,
        })))
        .into_response();
    }
}
