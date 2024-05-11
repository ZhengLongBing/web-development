use crate::set_sql_file;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table = Table::create()
            .table(Books::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Books::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Books::Title).string().not_null())
            .col(ColumnDef::new(Books::Isbn).string().not_null())
            .col(ColumnDef::new(Books::Author).string())
            .col(ColumnDef::new(Books::Category).string())
            .col(ColumnDef::new(Books::OnShelfTime).date().not_null())
            .col(ColumnDef::new(Books::Price).decimal_len(10,2).not_null())
            .to_owned();

        let sql = create_table.build(MysqlQueryBuilder);
        set_sql_file(&sql, file!(), "up");

        manager.create_table(create_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_table = Table::drop().table(Books::Table).if_exists().to_owned();

        let sql = drop_table.build(MysqlQueryBuilder);
        set_sql_file(&sql, file!(), "down");

        manager.drop_table(drop_table).await
    }
}

#[derive(Iden)]
pub(super) enum Books {
    #[iden = "Books"]
    Table,
    Id,
    Title,
    Isbn,
    Author,
    Category,
    OnShelfTime,
    Price,
}
