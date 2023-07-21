use ::entity::note;
use chrono::Utc;
use sea_orm::{prelude::Uuid, *};

use crate::types::types::NoteReq;

pub struct Mutation;

impl Mutation {
    pub async fn create_note(db: &DbConn, form_data: NoteReq) -> anyhow::Result<note::Model> {
        let model = note::ActiveModel {
            note: Set(form_data.note),
            duration_hours: Set(form_data.duration_hours),
            manual_password: Set(Some(form_data.manual_password)),
            notify_email: Set(Some(form_data.notify_email)),
            delete_at: Set(form_data.delete_at.unwrap()),
            ..Default::default()
        }
        .save(db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_note_by_id(db: &DbConn, id: Uuid) -> anyhow::Result<bool> {
        Ok(note::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_or_else(|_| false, |r| r.rows_affected == 1))
    }

    pub async fn delete_old_notes(db: &DbConn) -> anyhow::Result<bool> {
        Ok(note::Entity::delete_many()
            .filter(note::Column::DeleteAt.lt(Utc::now().naive_utc()))
            .exec(db)
            .await
            .map_or_else(|_| false, |r| r.rows_affected > 0))
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
