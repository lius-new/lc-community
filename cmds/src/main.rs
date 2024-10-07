use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

const APP_ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let app = lc_routes::build_api_root_router();
    let listener = tokio::net::TcpListener::bind(APP_ADDR).await.unwrap();

    tracing::info!("app server with: http://{}", APP_ADDR);
    axum::serve(listener, app).await.unwrap();
}
