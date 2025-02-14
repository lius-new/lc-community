use axum::{response::Result, Extension};
use lc_middlewares::auth;
use lc_utils::{
    errors::{AppError, RequestError},
    extract::Json,
    response::Response,
};

pub async fn view_by_hash(
    Json(payload): Json<lc_dto::articles::ArticleByHashRequestParams>,
) -> Result<Response<lc_models::articles::ArticleDetail>, AppError> {
    Ok(Response::default().success(
        "获取文章成功",
        Some(lc_services::articles::article_services::view_by_hash(&payload.hash).await?),
    ))
}

/// 创建文章
/// 该接口接收post + multipart请求参数
pub async fn create(
    state: Extension<auth::CurrentUser>,
    mut multipart: axum::extract::Multipart,
) -> Result<Response<()>, AppError> {
    let mut payload = lc_dto::articles::CreateArticleRequestParams {
        title: String::new(),
        description: String::new(),
        content: String::new(),
        tags: Vec::new(),
        groups: Vec::new(),
    };

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some("data") = field.name() {
            payload = serde_json::from_slice::<lc_dto::articles::CreateArticleRequestParams>(
                &field.bytes().await.map_err(|_| {
                    AppError::RequestError(RequestError::ParamNotFoundError("data".to_string()))
                })?,
            )?;
        } else {
            break;
        }
    }

    lc_services::articles::article_services::create(multipart, payload, &state.uuid).await?;
    Ok(Response::default().success("创建文章成功", Some(())))
}

pub async fn modify(
    state: Extension<auth::CurrentUser>,
    mut multipart: axum::extract::Multipart,
) -> Result<Response<()>, AppError> {
    let mut payload = lc_dto::articles::ModifyArticleRequestParams {
        title: String::new(),
        description: String::new(),
        content: String::new(),
        hash: String::new(),
        tags: Vec::new(),
        groups: Vec::new(),
    };

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some("data") = field.name() {
            payload = serde_json::from_slice::<lc_dto::articles::ModifyArticleRequestParams>(
                &field.bytes().await.map_err(|_| {
                    AppError::RequestError(RequestError::ParamNotFoundError("data".to_string()))
                })?,
            )?;
        } else {
            break;
        }
    }
    lc_services::articles::article_services::modify(multipart, payload, &state.uuid).await?;
    Ok(Response::default().success("修改文章成功", Some(())))
}

pub async fn delete_by_hash(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<lc_dto::articles::ArticleByHashRequestParams>,
) -> Result<Response<()>, AppError> {
    lc_services::articles::article_services::delete_by_hash(&payload.hash, &state.uuid).await?;
    Ok(Response::default().success("删除文章成功", Some(())))
}

pub async fn toggle_visiable(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<lc_dto::articles::ArticleByHashRequestParams>,
) -> Result<Response<()>, AppError> {
    lc_services::articles::article_services::toggle_visiable(&payload.hash, &state.uuid).await?;
    Ok(Response::default().success("删除文章成功", Some(())))
}

pub async fn page(
    Json(payload): Json<lc_dto::PageRequestParams>,
) -> Result<Response<lc_models::articles::ArticleByPage>, AppError> {
    Ok(Response::default().success(
        "查看文章成功",
        Some(
            lc_services::articles::article_services::view_by_page(
                payload.page_num,
                payload.page_size,
            )
            .await?,
        ),
    ))
}

pub async fn toplist(
    Json(payload): Json<lc_dto::PageRequestParams>,
) -> Result<Response<lc_models::articles::ArticleByPage>, AppError> {
    Ok(Response::default().success(
        "查看文章成功",
        Some(
            lc_services::articles::article_services::toplist(payload.page_num, payload.page_size)
                .await?,
        ),
    ))
}

pub async fn random(
    Json(payload): Json<lc_dto::PageRequestParams>,
) -> Result<Response<lc_models::articles::ArticleByPage>, AppError> {
    Ok(Response::default().success(
        "查看文章成功",
        Some(
            lc_services::articles::article_services::random(payload.page_num, payload.page_size)
                .await?,
        ),
    ))
}
