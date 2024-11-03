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
    pub total: i64,
}

pub mod article_groups {
    use super::*;

    /// 文章组
    #[derive(FromRow, Debug, Deserialize, Serialize)]
    pub struct ArticleGroup {
        pub name: String,
        pub description: String,
        pub visiable: bool,
    }

    /// 分页获取文章组
    #[derive(FromRow, Debug, Deserialize, Serialize)]
    pub struct ArticleGroupByPage {
        pub article_groups: Vec<ArticleGroup>,
        pub total: i64,
    }
}

pub mod article_tags {
    use super::*;

    /// 文章组
    #[derive(FromRow, Debug, Deserialize, Serialize)]
    pub struct ArticleTag {
        pub name: String,
        pub description: String,
        pub visiable: bool,
    }

    /// 分页获取文章组
    #[derive(FromRow, Debug, Deserialize, Serialize)]
    pub struct ArticleTagByPage {
        pub article_tags: Vec<ArticleTag>,
        pub total: i64,
    }
}
