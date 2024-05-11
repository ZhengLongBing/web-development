mod routes;

use axum::routing::get;
use axum::Router;
use db::connect::get_db_connect;
use std::env;
use tokio::net::TcpListener;
pub async fn serve() {
    let ip = env::var("IP").expect("没有找到环境变量 IP ！");
    let post = env::var("PORT").expect("没有找到环境变量 PORT ！");

    let db = get_db_connect().await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "hello world" }))
        .with_state(db);

    let listener = TcpListener::bind(format!("{ip}:{post}"))
        .await
        .expect("tcp 端口绑定错误！");

    axum::serve(listener, app).await.unwrap();
}
