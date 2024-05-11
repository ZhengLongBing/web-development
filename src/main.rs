use api;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("没有找到 .env 文件！");
    api::serve().await;
}
