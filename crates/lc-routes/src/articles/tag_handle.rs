use axum::extract::Path;
use lc_dto::articles::article_tags::{ArticleTagPageRequestParams, CreateArticleTagRequestParams};
use lc_models::articles::article_tags::{ArticleTag, ArticleTagByPage};
use lc_utils::{extract::Json, response::Response};

pub async fn create(Json(payload): Json<CreateArticleTagRequestParams>) -> Response<()> {
    let result = lc_services::articles::article_tags_services::create(payload).await;

    result.map_or_else(
        |e| Response::default().fail("创建文章标签失败", Some(e)),
        |v| Response::default().success("创建文章标签成功", Some(v)),
    )
}

pub async fn modify(Json(payload): Json<CreateArticleTagRequestParams>) -> Response<()> {
    let result = lc_services::articles::article_tags_services::modify(payload).await;

    result.map_or_else(
        |e| Response::default().fail("更新文章标签失败", Some(e)),
        |v| Response::default().success("更新文章标签成功", Some(v)),
    )
}

pub async fn delete(Path(id): Path<i32>) -> Response<()> {
    let result = lc_services::articles::article_tags_services::delete(id).await;

    result.map_or_else(
        |e| Response::default().fail("删除文章标签失败", Some(e)),
        |v| Response::default().success("删除文章标签成功", Some(v)),
    )
}

pub async fn toggle_visiable(Path(id): Path<i32>) -> Response<()> {
    let result = lc_services::articles::article_tags_services::toggle_visiable(id).await;

    result.map_or_else(
        |e| Response::default().fail("修改文章标签可见性失败", Some(e)),
        |v| Response::default().success("修改文章标签可见性成功", Some(v)),
    )
}

pub async fn view(Path(id): Path<i32>) -> Response<ArticleTag> {
    let article_tag = lc_services::articles::article_tags_services::view(id).await;

    article_tag.map_or_else(
        |e| Response::default().fail("获取文章标签失败", Some(e)),
        |v| Response::default().success("获取文章标签成功", Some(v)),
    )
}

pub async fn view_by_page(
    Json(payload): Json<ArticleTagPageRequestParams>,
) -> Response<ArticleTagByPage> {
    let article_tags = lc_services::articles::article_tags_services::view_by_page(
        payload.page_size,
        payload.page_num,
    )
    .await;

    article_tags.map_or_else(
        |e| Response::default().fail("获取文章标签失败", Some(e)),
        |v| Response::default().success("获取文章标签成功", Some(v)),
    )
}
