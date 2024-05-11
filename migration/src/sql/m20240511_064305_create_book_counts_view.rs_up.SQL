CREATE VIEW `BookCounts` AS
SELECT
    `Books`.`title`,
    `Books`.`isbn`,
    `Books`.`author`,
    `Books`.`category`,
    `Books`.`on_shelf_time`,
    `Books`.`price`,
    COALESCE(`BookGroups`.count, 0) AS count
FROM
    (
        SELECT `book_id`, COUNT(*) AS count
        FROM `BookManagers`
        GROUP BY `book_id`
    ) AS `BookGroups`
RIGHT JOIN `Books` ON `Books`.`id` = `BookGroups`.`book_id`;
