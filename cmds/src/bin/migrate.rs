use lc_utils::config::AppCon;
use sqlx::migrate::Migrator;
use std::path::Path;
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

    let (database_url, database_max_connections) = (
        AppCon.database.url.as_str(),
        AppCon.database.max_connections,
    );
    let pool = lc_utils::database::init_db(database_url, database_max_connections).await;

    let create_table_count = std::fs::read_dir("./migrations/")?.count();
    tracing::info!("create table with count({}) tables", create_table_count);

    //执行表格创建迁移

    //command::migrate(AppCon.database.migrate.current.as_ref(), pool.get().await).await;
    let sqls_path = "./migrations";
    let m = Migrator::new(Path::new(sqls_path))
        .await
        .expect(format!("{:?} is not exist!", Path::new(sqls_path)).as_str());

    m.run(pool.get().await).await.unwrap();

    println!("{}", lc_utils::uuid());

    Ok(())
}
