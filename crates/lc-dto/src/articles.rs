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

/// 该类型用于需要接收一个hash请求参数的类型
#[derive(Deserialize, Serialize, Debug)]
pub struct ArticleByHashRequestParams {
    pub hash: String,
}

pub mod article_groups {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct CreateArticleGroupRequestParams {
        pub name: String,
        pub description: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ModifyArticleGroupRequestParams {
        pub id: i32,
        pub name: String,
        pub description: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ArticleGroupPageRequestParams {
        pub page_num: i32,
        pub page_size: i32,
    }
}

pub mod article_tags {
    use super::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct CreateArticleTagRequestParams {
        pub name: String,
        pub description: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ModifyArticleTagRequestParams {
        pub id: i32,
        pub name: String,
        pub description: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ArticleTagPageRequestParams {
        pub page_num: i32,
        pub page_size: i32,
    }
}
