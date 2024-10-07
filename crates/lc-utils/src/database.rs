use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config;
use tokio::sync::OnceCell;

pub static DB: OnceCell<Arc<Database>> = OnceCell::const_new();

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database(Pool<Postgres>);

impl Database {
    pub async fn new() -> Self {
        let (url, max_connections) = (
            config::AppCon.database.url.as_str(),
            config::AppCon.database.max_connections,
        );

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await
            .unwrap();

        Self(pool)
    }
    pub async fn get(&self) -> &Pool<Postgres> {
        &self.0
    }
}
