pub use sea_orm_migration::prelude::*;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

mod entity;

mod m20240511_061642_create_users_table;
mod m20240511_061707_create_books_table;
mod m20240511_061941_create_book_managers_table;
mod m20240511_063003_create_borrow_records_table;
mod m20240511_064305_create_book_counts_view;
mod m20240511_073422_create_available_book_counts_view;
mod m20240511_083253_init_db;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240511_061642_create_users_table::Migration),
            Box::new(m20240511_061707_create_books_table::Migration),
            Box::new(m20240511_061941_create_book_managers_table::Migration),
            Box::new(m20240511_063003_create_borrow_records_table::Migration),
            Box::new(m20240511_064305_create_book_counts_view::Migration),
            Box::new(m20240511_073422_create_available_book_counts_view::Migration),
            Box::new(m20240511_083253_init_db::Migration),
        ]
    }
}

pub(crate) fn get_model_from_json<T: DeserializeOwned>(content: &str) -> Vec<T> {
    let models = serde_json::from_str::<serde_json::Value>(content).expect("json 解析错误！");
    models
        .as_array()
        .expect("解析的json不是数组类型")
        .into_iter()
        .map(|model| serde_json::from_value::<T>(model.clone()).expect("model 反序列化错误！"))
        .collect()
}

pub(crate) fn set_sql_file(sql: &str, file: &str, suffix: &str) {
    // Convert string to `Path` and safely handle potential None values from `parent()` and `file_name()`
    let file_path = Path::new(file);
    let parent_path = file_path.parent().expect("Failed to find parent directory");
    let file_name = file_path.file_name().expect("Failed to extract file name");

    // Create the path to the directory where the SQL file will be stored
    let mut path = PathBuf::from(parent_path);
    path.push("sql");  // Use push to correctly handle path separators

    // Construct file name using provided suffix and ensure uppercase extension for consistency
    let migrate_file_name = format!("{}_{}.SQL", file_name.to_string_lossy(), suffix);
    path.push(migrate_file_name);

    // Attempt to create and write to the file, handling potential errors
    let mut file = File::create(&path)
        .unwrap_or_else(|err| panic!("Failed to create file {:?}: {}", path, err));
    file.write_all(sql.as_bytes())
        .unwrap_or_else(|err| panic!("Failed to write to file {:?}: {}", path, err));
}
