use axum::extract::Path;
use axum::response::Result;
use lc_services::articles::article_groups_services;
use lc_utils::{errors::AppError, extract::Json, response::Response};

use lc_dto::articles::article_groups::{
    ArticleGroupPageRequestParams, CreateArticleGroupRequestParams, ModifyArticleGroupRequestParams,
};
use lc_models::articles::article_groups::{ArticleGroup, ArticleGroupByPage};

pub async fn create(
    Json(payload): Json<CreateArticleGroupRequestParams>,
) -> Result<Response<()>, AppError> {
    article_groups_services::create(payload).await?;

    Ok(Response::default().success("创建文章成功", Some(())))
}

pub async fn modify(
    Json(payload): Json<ModifyArticleGroupRequestParams>,
) -> Result<Response<()>, AppError> {
    article_groups_services::modify(payload).await?;

    Ok(Response::default().success("更新文分组章成功", Some(())))
}

pub async fn delete(Path(id): Path<i32>) -> Result<Response<()>, AppError> {
    article_groups_services::delete(id).await?;
    Ok(Response::default().success("删除文章分组章成功", Some(())))
}

pub async fn toggle_visiable(Path(id): Path<i32>) -> Result<Response<()>, AppError> {
    article_groups_services::toggle_visiable(id).await?;

    Ok(Response::default().success("修改文章分组可见性成功", Some(())))
}

pub async fn view(Path(id): Path<i32>) -> Result<Response<ArticleGroup>, AppError> {
    let article_group = article_groups_services::view(id).await?;

    Ok(Response::default().success("获取文章分组成功", Some(article_group)))
}

pub async fn view_by_page(
    Json(payload): Json<ArticleGroupPageRequestParams>,
) -> Result<Response<ArticleGroupByPage>, AppError> {
    let article_groups =
        article_groups_services::view_by_page(payload.page_num, payload.page_size).await?;

    Ok(Response::default().success("获取文章分组成功", Some(article_groups)))
}
