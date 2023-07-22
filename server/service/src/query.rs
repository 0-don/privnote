use ::entity::{note, note::Entity as Note};
use sea_orm::{DbConn, DbErr, EntityTrait};

pub struct Query;

impl Query {
    pub async fn find_note_by_id(db: &DbConn, id: String) -> Result<Option<note::Model>, DbErr> {
        let model = Note::find_by_id(id).one(db).await;

        Ok(model.unwrap())
    }

    // /// If ok, returns (posts models, num pages).
    // pub async fn find_posts_in_page(
    //     db: &DbConn,
    //     page: u64,
    //     posts_per_page: u64,
    // ) -> Result<(Vec<posts::Model>, u64), DbErr> {
    //     // Setup paginator
    //     let paginator = Post::find()
    //         .order_by_asc(posts::Column::Id)
    //         .paginate(db, posts_per_page);
    //     let num_pages = paginator.num_pages().await?;

    //     // Fetch paginated posts
    //     paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    // }
}
