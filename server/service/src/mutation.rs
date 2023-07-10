use ::entity::{note, note::Entity as Note};
use sea_orm::*;

use crate::types::types::NoteReq;

pub struct Mutation;

impl Mutation {
    pub async fn create_note(db: &DbConn, form_data: NoteReq) -> Result<bool, DbErr> {
        let model = note::Model {
            note: form_data.note,
            duration_hours: form_data.duration_hours,
            manual_password: Some(form_data.manual_password),
            notify_email: Some(form_data.notify_email),
            notify_ref: Some(form_data.notify_ref),

            id: Default::default(),
            created_at: Default::default(),
        };

        // let active_model: note::ActiveModel = note::ActiveModel {
        //     note: Set(model.note),
        //     ..Default::default()
        // };

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
