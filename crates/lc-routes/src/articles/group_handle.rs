use axum::extract::Path;
use lc_utils::{extract::Json, response::Response};

use lc_dto::articles::article_groups::{
    ArticleGroupPageRequestParams, CreateArticleGroupRequestParams,
};
use lc_models::articles::article_groups::{ArticleGroup, ArticleGroupByPage};

pub async fn create(Json(payload): Json<CreateArticleGroupRequestParams>) -> Response<()> {
    let result = lc_services::articles::article_groups_services::create(payload).await;

    result.map_or_else(
        |e| Response::default().fail("创建文章分组失败", Some(e)),
        |v| Response::default().success("创建文章成功", Some(v)),
    )
}

pub async fn modify(Json(payload): Json<CreateArticleGroupRequestParams>) -> Response<()> {
    let result = lc_services::articles::article_groups_services::modify(payload).await;

    result.map_or_else(
        |e| Response::default().fail("更新文章分组失败", Some(e)),
        |v| Response::default().success("更新文分组章成功", Some(v)),
    )
}

pub async fn delete(Path(id): Path<i32>) -> Response<()> {
    let result = lc_services::articles::article_groups_services::delete(id).await;

    result.map_or_else(
        |e| Response::default().fail("更新文章分组失败", Some(e)),
        |v| Response::default().success("更新文章分组成功", Some(v)),
    )
}

pub async fn toggle_visiable(Path(id): Path<i32>) -> Response<()> {
    let result = lc_services::articles::article_groups_services::toggle_visiable(id).await;

    result.map_or_else(
        |e| Response::default().fail("修改文章分组可见性失败", Some(e)),
        |v| Response::default().success("修改文章分组可见性成功", Some(v)),
    )
}

pub async fn view(Path(id): Path<i32>) -> Response<ArticleGroup> {
    let article_group = lc_services::articles::article_groups_services::view(id).await;

    article_group.map_or_else(
        |e| Response::default().fail("获取文章分组失败", Some(e)),
        |v| Response::default().success("获取文章分组成功", Some(v)),
    )
}

pub async fn view_by_page(
    Json(payload): Json<ArticleGroupPageRequestParams>,
) -> Response<ArticleGroupByPage> {
    let article_groups = lc_services::articles::article_groups_services::view_by_page(
        payload.page_size,
        payload.page_num,
    )
    .await;

    article_groups.map_or_else(
        |e| Response::default().fail("获取文章分组失败", Some(e)),
        |v| Response::default().success("获取文章分组成功", Some(v)),
    )
}
