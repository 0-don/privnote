use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use core::time::Duration;
use entity::note;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use moka::future::Cache;
use service::{types::types::CsrfToken, Mutation as MutationCore};
use std::env;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    constants,
    model::response::{ResponseBody, ResponseMessages},
    DB,
};

use super::types::AppState;

pub async fn get_app_state() -> AppState {
    AppState {
        conn: get_db_connection().await.unwrap(),
        cache: Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(1000))
            .build(),
    }
}

pub async fn cron_delete_old_notes() -> anyhow::Result<()> {
    let sched = JobScheduler::new().await?;

    DB.set(get_db_connection().await.unwrap())
        .expect("error db connection");
    // sec   min   hour   day of month   month   day of week   year
    // *     *     *      *              *       *             *
    sched
        .add(Job::new_repeated_async(
            Duration::from_secs(600),
            |_uuid, _l| {
                Box::pin(async {
                    let db = DB.get().unwrap();
                    MutationCore::delete_old_notes(db).await.unwrap();
                })
            },
        )?)
        .await?;

    sched.start().await?;
    Ok(())
}

pub async fn get_db_connection() -> anyhow::Result<DatabaseConnection> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    println!("Connecting to database: {db_url}", db_url = db_url);

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    Ok(conn)
}

pub async fn check_csrf_token(captcha: CsrfToken, state: &State<AppState>) -> Option<Response> {
    let text = state.cache.get(&captcha.tag);
    if text.is_none() {
        return Some(
            Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
                constants::MESSAGE_NO_TAG_OR_NO_TEXT.to_string(),
                constants::ERROR_PATH.to_string(),
            )))
            .into_response(),
        );
    }

    if captcha.text != text.unwrap() {
        return Some(
            Json(ResponseBody::<bool>::new_msg(ResponseMessages::new(
                constants::MESSAGE_CAPTCHA_WRONG.to_string(),
                constants::ERROR_PATH.to_string(),
            )))
            .into_response(),
        );
    }

    None
}

pub async fn send_email(note: &note::Model) -> anyhow::Result<bool> {
    let host = env::var("SMTP_HOST").expect("SMTP_HOST is not set in .env file");
    let port = env::var("SMTP_PORT").expect("SMTP_PORT is not set in .env file");
    let user = env::var("SMTP_USER").expect("SMTP_USER is not set in .env file");
    let pass = env::var("SMTP_PASS").expect("SMTP_PASS is not set in .env file");

    let email = Message::builder()
        .from(format!("Privnote <{}>", user).parse().unwrap())
        .to(note.notify_email.as_ref().unwrap().parse().unwrap())
        .subject("Your Privnote has been read")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Hello world?"))
        .unwrap();

    let creds = Credentials::new(user.to_owned(), pass.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(host.as_str())
        .unwrap()
        .credentials(creds)
        .port(port.parse::<u16>().unwrap())
        .build();
    println!("Sending email to: {}", note.notify_email.as_ref().unwrap());
    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(true),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
