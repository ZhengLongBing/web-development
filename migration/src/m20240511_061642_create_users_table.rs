use crate::set_sql_file;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table = Table::create()
            .table(Users::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Users::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Users::Username).string().not_null())
            .col(
                ColumnDef::new(Users::Account)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(Users::Email)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(Users::Password).string().not_null())
            .col(ColumnDef::new(Users::RegisterTime).date_time().not_null())
            .to_owned();

        let sql = create_table.build(MysqlQueryBuilder);
        set_sql_file(&sql, file!(), "up");

        manager.create_table(create_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_table = Table::drop().table(Users::Table).if_exists().to_owned();

        let sql = drop_table.build(MysqlQueryBuilder);
        set_sql_file(&sql, file!(), "down");

        manager.drop_table(drop_table).await
    }
}

#[derive(Iden)]
pub(super) enum Users {
    #[iden = "Users"]
    Table,
    Id,
    Username,
    Account,
    Email,
    Password,
    RegisterTime,
}
