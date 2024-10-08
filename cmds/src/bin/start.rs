use lc_utils::config::AppCon;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// if you want to run app, please confirm:
/// 1.config/default.toml is exist. (crates/lc-utils/config.rs file need init)
/// 2.postgresql running. please see config/default.toml and crates/lc-utils/database.rs
/// 3.port is not use
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
