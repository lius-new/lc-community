use lc_utils::config::AppCon;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let service_url = format!("127.0.0.1:{}", AppCon.service.port);

    let app = lc_routes::build_api_root_router();
    let listener = tokio::net::TcpListener::bind(service_url.as_str())
        .await
        .unwrap();

    tracing::info!("app server with: http://{}", service_url);
    axum::serve(listener, app).await.unwrap();
}
