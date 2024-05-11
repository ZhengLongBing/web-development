use sea_orm_migration::prelude::*;
use crate::set_sql_file;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"
CREATE VIEW `AvailableBookCounts` AS
SELECT
    `Books`.`title`,
    `Books`.`isbn`,
    `Books`.`author`,
    `Books`.`category`,
    `Books`.`on_shelf_time`,
    `Books`.`price`,
    `AvailableBookGroups`.count
FROM
    (
        SELECT `book_id`, COUNT(*) AS count
        FROM `BookManagers`
        WHERE book_status='AVAILABLE'
        GROUP BY `book_id`
    ) AS `AvailableBookGroups`
        JOIN `Books` ON `Books`.`id` = `AvailableBookGroups`.`book_id`;
"#;
        set_sql_file(sql, file!(), "up");
        db.execute_unprepared(sql).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"DROP VIEW IF EXISTS `availablebookcounts`"#;
        set_sql_file(sql, file!(), "down");
        db.execute_unprepared(sql).await?;
        Ok(())
    }
}
