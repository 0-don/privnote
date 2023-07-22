use ::entity::{note, note::Entity as Note};
use sea_orm::{DbConn, EntityTrait};

pub struct Query;

impl Query {
    pub async fn find_note_by_id(db: &DbConn, id: String) -> anyhow::Result<Option<note::Model>> {
        let model = Note::find_by_id(id).one(db).await;

        Ok(model.unwrap())
    }
}
