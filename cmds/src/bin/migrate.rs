use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// support migrate step:
/// 1.command: `cargo add sqlx`, need  migrate features
/// 2.`cargo install sqlx-cli`
/// 3.`mkdir migrations && cd migrations`
/// 4.`sqlx migrate add migration_file_name` (example: `sqlx migrate add create_users_table`)
/// 5.coding sql into generate files.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let pool = lc_utils::database::init_db().await;

    let create_table_count = std::fs::read_dir("./migrations/creates/")?.count();
    tracing::info!("create table with count({}) tables", create_table_count);

    //执行表格创建迁移
    sqlx::migrate!("../migrations/creates")
        .run(pool.get().await)
        .await?;

    Ok(())
}
