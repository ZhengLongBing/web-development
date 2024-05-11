use crate::set_sql_file;
use sea_orm_migration::prelude::*;
use crate::m20240511_061941_create_book_managers_table::BookManagers;
use crate::m20240511_061642_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table = TableCreateStatement::new()
            .table(BorrowRecords::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(BorrowRecords::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(BorrowRecords::BookId).integer().not_null())
            .col(ColumnDef::new(BorrowRecords::UserId).integer().not_null())
            .col(ColumnDef::new(BorrowRecords::BorrowTime).date().not_null())
            .col(ColumnDef::new(BorrowRecords::ReturnTime).date())
            .col(
                ColumnDef::new(BorrowRecords::BorrowStatus)
                    .enumeration(
                        BorrowRecords::BorrowStatus,
                        vec![
                            BorrowStatus::Borrowed,
                            BorrowStatus::Returned,
                            BorrowStatus::Overdue,
                        ],
                    )
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(BorrowRecords::Table, BorrowRecords::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(BorrowRecords::Table, BorrowRecords::BookId)
                    .to(BookManagers::Table, BookManagers::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned();

        let sql = create_table.build(MysqlQueryBuilder);
        set_sql_file(&sql, file!(), "up");

        manager.create_table(create_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_table = Table::drop()
            .table(BorrowRecords::Table)
            .if_exists()
            .to_owned();

        let sql = drop_table.build(MysqlQueryBuilder);
        set_sql_file(&sql, file!(), "down");

        manager.drop_table(drop_table).await
    }
}

#[derive(Iden)]
enum BorrowRecords {
    #[iden = "BorrowRecords"]
    Table,
    Id,
    BookId,
    UserId,
    BorrowTime,
    ReturnTime,
    BorrowStatus,
}

#[derive(Iden)]
enum BorrowStatus {
    #[iden = "BORROWED"]
    Borrowed,
    #[iden = "RETURNED"]
    Returned,
    #[iden = "OVERDUE"]
    Overdue,
}
