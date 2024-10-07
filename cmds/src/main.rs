use std::sync::Arc;

use lc_utils::{config::AppCon, database::Database};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let service_url = format!("127.0.0.1:{}", AppCon.service.port);

    let db = lc_utils::database::DB
        .get_or_init(|| async { Arc::new(Database::new().await) })
        .await;

    let pool = db.get().await;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool)
        .await
        .unwrap();
    println!("{:?}", row);

    let app = lc_routes::build_api_root_router();
    let listener = tokio::net::TcpListener::bind(service_url.as_str())
        .await
        .unwrap();

    tracing::info!("app server with: http://{}", service_url);
    axum::serve(listener, app).await.unwrap();
}
