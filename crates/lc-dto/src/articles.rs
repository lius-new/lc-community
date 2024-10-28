use serde::{Deserialize, Serialize};

/// 创建文章的请求参数
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateArticleRequestParams {
    pub title: String,
    pub description: String,
    pub content: String,
    pub tags: Vec<i32>,
    pub groups: Vec<i32>,
}

/// 创建文章的请求参数
#[derive(Deserialize, Serialize, Debug)]
pub struct ArticlePageRequestParams {
    pub page_num: i32,
    pub page_size: i32,
}

/// 更新文章的请求参数
#[derive(Deserialize, Serialize, Debug)]
pub struct ModifyArticleRequestParams {
    pub title: String,
    pub description: String,
    pub content: String,
    pub tags: Vec<i32>,
    pub groups: Vec<i32>,
    pub hash: String,
}
