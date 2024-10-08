use lc_utils::config::AppCon;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let server_url = format!("127.0.0.1:{}", AppCon.service.port);

    lc_utils::database::init_db().await;

    let app = lc_routes::build_api_root_router();
    let listener = tokio::net::TcpListener::bind(server_url.as_str()).await?;

    tracing::info!("app server with: http://{}", server_url);
    axum::serve(listener, app).await?;

    Ok(())
}
