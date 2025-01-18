use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Packets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Packets::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Packets::Uuid).uuid().not_null())
                    .col(ColumnDef::new(Packets::Bytes).binary().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Packets::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Packets {
    Table,
    Id,
    Uuid,
    Bytes,
}
