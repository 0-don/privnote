use entity::note;
use sea_orm::{DbConn, DbErr, Set};

use crate::types::types::NoteReq;

pub struct Mutation;

impl Mutation {
    pub async fn create_note(db: &DbConn, form_data: NoteReq) -> Result<note::Model, DbErr> {
        let res = note::Model {
            note: form_data.note,
            id: todo!(),
            duration_hours: todo!(),
            manual_password: todo!(),
            notify_email: todo!(),
            notify_ref: todo!(),
            created_at: todo!(),
        };

        // let active_model = note::ActiveModel {
        //     notify_email: Set(None),
        //     ..Default::default()
        // };

        Ok(res)
    }

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
