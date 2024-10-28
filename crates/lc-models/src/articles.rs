use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 文章详情
#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct ArticleDetail {
    pub title: String,
    pub description: String,
    pub content: String,
    pub covers: Option<Vec<String>>,
    pub created_at: Vec<String>,
}

/// 文章
#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct Article {
    pub title: String,
    pub description: String,
}

/// 分页获取文章
#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct ArticleByPage {
    pub articles: Vec<Article>,
    pub total: i32,
}
