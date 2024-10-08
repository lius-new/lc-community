use crate::config::AppCon;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tokio::sync::OnceCell;

///  数据库连接池对象: 需要在async 方法中调用get_or_init方法来初始化
pub static DB: OnceCell<Arc<Database>> = OnceCell::const_new();

/// 数据库连接对象初始化
pub async fn init_db() -> &'static Arc<Database> {
    let (database_url, database_max_connections) = (
        AppCon.database.url.as_str(),
        AppCon.database.max_connections,
    );

    DB.get_or_init(|| async {
        let database = Database::new(database_url, database_max_connections)
            .await
            .unwrap();
        Arc::new(database)
    })
    .await
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database(Pool<Postgres>);

impl Database {
    /// 数据库连接对象构建
    pub async fn new(url: &str, max_connections: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;

        Ok(Self(pool))
    }

    /// 获取数据库连接
    pub async fn get(&self) -> &Pool<Postgres> {
        &self.0
    }
}
