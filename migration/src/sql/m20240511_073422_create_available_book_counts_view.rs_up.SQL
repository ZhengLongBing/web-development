
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
