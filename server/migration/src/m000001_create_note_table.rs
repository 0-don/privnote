use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum Note {
    Table,
    Id,
    Note,
    DurationHours,
    ManualPassword,
    NotifyEmail,
    NotifyRef,
    CreatedAt,
    DeleteAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Note::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Note::Id)
                            .uuid()
                            .unique_key()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".into()),
                    )
                    .col(ColumnDef::new(Note::Note).string().string().not_null())
                    .col(
                        ColumnDef::new(Note::DurationHours)
                            .integer()
                            .not_null()
                            .default(0)
                            .extra("check (duration_hours between 0 and 720)".into()),
                    )
                    .col(ColumnDef::new(Note::ManualPassword).string())
                    .col(ColumnDef::new(Note::NotifyEmail).string())
                    .col(ColumnDef::new(Note::NotifyRef).string())
                    .col(
                        ColumnDef::new(Note::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT now()".into()),
                    )
                    .col(
                        ColumnDef::new(Note::DeleteAt)
                            .timestamp()
                            .extra("DEFAULT now()".into()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Note::Table).to_owned())
            .await
    }
}
