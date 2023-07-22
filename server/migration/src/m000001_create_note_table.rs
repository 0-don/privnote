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
    DestroyWithoutConfirmation,
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
                            .string()
                            .string_len(8)
                            .unique_key()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Note::Note)
                            .string()
                            .string_len(1000000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Note::DurationHours)
                            .integer()
                            .not_null()
                            .default(0)
                            .extra("check (duration_hours between 0 and 720)".into()),
                    )
                    .col(
                        ColumnDef::new(Note::ManualPassword)
                            .string()
                            .string_len(1000),
                    )
                    .col(ColumnDef::new(Note::NotifyEmail).string().string_len(1000))
                    .col(
                        ColumnDef::new(Note::DestroyWithoutConfirmation)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Note::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default("now()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Note::DeleteAt)
                            .timestamp()
                            .not_null()
                            .default("now()".to_owned()),
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
