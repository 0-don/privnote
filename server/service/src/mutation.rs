use ::entity::note;
use chrono::Utc;
use sea_orm::*;

use crate::types::types::NoteReq;

pub struct Mutation;

impl Mutation {
    pub async fn create_note(db: &DbConn, form_data: NoteReq) -> Result<note::Model, DbErr> {
        let delete_at =
            (Utc::now() + chrono::Duration::hours(form_data.duration_hours as i64)).naive_utc();

        let model = note::ActiveModel {
            note: Set(form_data.note),
            duration_hours: Set(form_data.duration_hours),
            manual_password: Set(Some(form_data.manual_password)),
            notify_email: Set(Some(form_data.notify_email)),
            notify_ref: Set(Some(form_data.notify_ref)),
            delete_at: Set(Some(delete_at)),
            ..Default::default()
        }
        .save(db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn destroy_note_by_id(db: &DbConn, note: &note::Model) -> Result<bool, DbErr> {
        let result = note::ActiveModel {
            id: Set(note.id),
            ..Default::default()
        }
        .delete(db)
        .await?;

        if result.rows_affected == 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn delete_old_notes(db: &DbConn) -> anyhow::Result<bool> {
        Ok(true)
    }

    // pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     Note::delete_many().exec(db).await
    // }

    // pub async fn update_post_by_id(
    //     db: &DbConn,
    //     id: i32,
    //     form_data: posts::Model,
    // ) -> Result<posts::Model, DbErr> {
    //     let posts: posts::ActiveModel = Post::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find posts.".to_owned()))
    //         .map(Into::into)?;

    //     posts::ActiveModel {
    //         id: posts.id,
    //         title: Set(form_data.title.to_owned()),
    //         text: Set(form_data.text.to_owned()),
    //     }
    //     .update(db)
    //     .await
    // }

    // pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    //     let posts: posts::ActiveModel = Post::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find posts.".to_owned()))
    //         .map(Into::into)?;

    //     posts.delete(db).await
    // }

    // pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     Post::delete_many().exec(db).await
    // }
}
