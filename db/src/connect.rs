use common::error::db_err::DbErr;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

pub async fn get_db_connect() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL").map_err(|_| DbErr::NoFindDbURL)?;
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(25)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    Database::connect(opt)
        .await
        .map_err(|err| DbErr::DbConnectErr(err))
}
