use crate::set_sql_file;
use sea_orm_migration::prelude::*;
use crate::m20240511_061707_create_books_table::Books;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table = Table::create()
            .table(BookManagers::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(BookManagers::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(BookManagers::BookId).integer().not_null()
            )
            .col(
                ColumnDef::new(BookManagers::BookStatus)
                    .enumeration(
                        BookManagers::BookStatus,
                        vec![
                            BookStatus::Available,
                            BookStatus::Borrowed,
                            BookStatus::OnHold,
                            BookStatus::Maintenance,
                        ],
                    )
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(BookManagers::Table, BookManagers::BookId)
                    .to(Books::Table, Books::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned();

        let sql = create_table.build(MysqlQueryBuilder);
        set_sql_file(&sql, file!(), "up");

        manager.create_table(create_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_table = Table::drop().table(BookManagers::Table).if_exists().to_owned();

        let sql = drop_table.build(MysqlQueryBuilder);
        set_sql_file(&sql, file!(), "down");

        manager.drop_table(drop_table).await
    }
}
#[derive(Iden)]
pub(super) enum BookManagers {
    #[iden = "BookManagers"]
    Table,
    Id,
    BookId,
    BookStatus
}

#[derive(Iden)]
enum BookStatus {
    #[iden = "AVAILABLE"]
    Available,
    #[iden = "BORROWED"]
    Borrowed,
    #[iden = "ON_HOLD"]
    OnHold,
    #[iden = "MAINTENANCE"]
    Maintenance,
}