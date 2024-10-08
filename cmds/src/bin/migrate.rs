use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let pool = lc_utils::database::init_db().await;

    sqlx::migrate!("../migrations")
        .run(pool.get().await)
        .await?;

    Ok(())
}
