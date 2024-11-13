use axum::{extract::Path, response::Result};
use lc_dto::articles::article_tags::{
    ArticleTagPageRequestParams, CreateArticleTagRequestParams, ModifyArticleTagRequestParams,
};
use lc_models::articles::article_tags::{ArticleTag, ArticleTagByPage};
use lc_services::articles::article_tags_services;
use lc_utils::{errors::AppError, extract::Json, response::Response};

pub async fn create(
    Json(payload): Json<CreateArticleTagRequestParams>,
) -> Result<Response<()>, AppError> {
    article_tags_services::create(payload).await?;

    Ok(Response::default().success("创建文章标签成功", Some(())))
}

pub async fn modify(
    Json(payload): Json<ModifyArticleTagRequestParams>,
) -> Result<Response<()>, AppError> {
    article_tags_services::modify(payload).await?;
    Ok(Response::default().success("更新文章标签成功", Some(())))
}

pub async fn delete(Path(id): Path<i32>) -> Result<Response<()>, AppError> {
    article_tags_services::delete(id).await?;

    Ok(Response::default().success("删除文章标签成功", Some(())))
}

pub async fn toggle_visiable(Path(id): Path<i32>) -> Result<Response<()>, AppError> {
    article_tags_services::toggle_visiable(id).await?;

    Ok(Response::default().success("修改文章标签可见性成功", Some(())))
}

pub async fn view(Path(id): Path<i32>) -> Result<Response<ArticleTag>, AppError> {
    let article_tag = article_tags_services::view(id).await?;

    Ok(Response::default().success("获取文章标签成功", Some(article_tag)))
}

pub async fn view_by_page(
    Json(payload): Json<ArticleTagPageRequestParams>,
) -> Result<Response<ArticleTagByPage>, AppError> {
    let article_tags =
        article_tags_services::view_by_page(payload.page_size, payload.page_num).await?;

    Ok(Response::default().success("获取文章标签成功", Some(article_tags)))
}
