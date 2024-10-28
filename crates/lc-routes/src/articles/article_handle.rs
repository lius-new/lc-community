use axum::http::StatusCode;
use lc_utils::{extract::Json, response::Response};

pub async fn view_by_hash() -> Response<lc_models::articles::ArticleDetail> {
    let article = lc_services::articles::article_services::view_by_hash("abc").await;

    article.map_or_else(
        |e| Response::default().fail("获取文章失败", Some(e)),
        |v| Response::default().success("获取文章成功", Some(v)),
    )
}

/// 创建文章
/// 该接口接收post + multipart请求参数
pub async fn create(mut multipart: axum::extract::Multipart) -> Response<()> {
    let mut payload = lc_dto::articles::CreateArticleRequestParams {
        title: String::new(),
        description: String::new(),
        content: String::new(),
        tags: Vec::new(),
        groups: Vec::new(),
    };

    // TODO: 待重写
    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("data") => match field.bytes().await {
                Ok(data) => match serde_json::from_slice::<
                    lc_dto::articles::CreateArticleRequestParams,
                >(&data)
                {
                    Ok(value) => payload = value,
                    Err(_) => {
                        return Response::default()
                            .with_status_code(StatusCode::BAD_REQUEST)
                            .fail("请求参数错误", None)
                    }
                },

                Err(e) => return e.into(),
            },
            Some(_) => break,
            None => break,
        }
    }

    let article = lc_services::articles::article_services::create(multipart, payload).await;

    article.map_or_else(
        |e| Response::default().fail("创建文章失败", Some(e)),
        |v| Response::default().success("创建文章成功", Some(v)),
    )
}

pub async fn modify(mut multipart: axum::extract::Multipart) -> Response<()> {
    let mut payload = lc_dto::articles::ModifyArticleRequestParams {
        title: String::new(),
        description: String::new(),
        content: String::new(),
        hash: String::new(),
        tags: Vec::new(),
        groups: Vec::new(),
    };

    // TODO: 待重写
    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("title") => {
                match field.text().await {
                    Ok(value) => payload.title = value,
                    Err(e) => return e.into(),
                };
            }
            Some("description") => {
                match field.text().await {
                    Ok(value) => payload.description = value,
                    Err(e) => return e.into(),
                };
            }
            Some("content") => {
                match field.text().await {
                    Ok(value) => payload.content = value,
                    Err(e) => return e.into(),
                };
            }
            Some("hash") => {
                match field.text().await {
                    Ok(value) => payload.hash = value,
                    Err(e) => return e.into(),
                };
            }
            Some("tags") => {
                match field.text().await {
                    Ok(value) => match value.parse::<i32>() {
                        Ok(v) => payload.tags.push(v),
                        Err(_) => {
                            return Response::default()
                                .with_status_code(StatusCode::BAD_REQUEST)
                                .fail("请求参数错误", None)
                        }
                    },
                    Err(e) => return e.into(),
                };
            }
            Some("groups") => {
                match field.text().await {
                    Ok(value) => match value.parse::<i32>() {
                        Ok(v) => payload.groups.push(v),
                        Err(_) => {
                            return Response::default()
                                .with_status_code(StatusCode::BAD_REQUEST)
                                .fail("请求参数错误", None)
                        }
                    },
                    Err(e) => return e.into(),
                };
            }
            Some(_) => break,
            None => break,
        }
    }

    lc_services::articles::article_services::modify(payload)
        .await
        .map_or_else(
            |e| Response::default().fail("修改文章失败", Some(e)),
            |v| Response::default().success("修改文章成功", Some(v)),
        )
}

pub async fn delete_by_hash() -> Response<()> {
    let article = lc_services::articles::article_services::delete_by_hash("abc").await;

    article.map_or_else(
        |e| Response::default().fail("删除文章失败", Some(e)),
        |v| Response::default().success("删除文章成功", Some(v)),
    )
}

pub async fn toggle_visiable() -> Response<()> {
    let article = lc_services::articles::article_services::toggle_visiable("abc").await;

    article.map_or_else(
        |e| Response::default().fail("更改文章可见性失败", Some(e)),
        |v| Response::default().success("删除文章可见性成功", Some(v)),
    )
}

pub async fn page(
    Json(payload): Json<lc_dto::articles::ArticlePageRequestParams>,
) -> Response<lc_models::articles::ArticleByPage> {
    let article =
        lc_services::articles::article_services::page(payload.page_num, payload.page_size).await;

    article.map_or_else(
        |e| Response::default().fail("查看文章失败", Some(e)),
        |v| Response::default().success("查看文章成功", Some(v)),
    )
}

pub async fn toplist(
    Json(payload): Json<lc_dto::articles::ArticlePageRequestParams>,
) -> Response<lc_models::articles::ArticleByPage> {
    let article =
        lc_services::articles::article_services::toplist(payload.page_num, payload.page_size).await;

    article.map_or_else(
        |e| Response::default().fail("查看文章失败", Some(e)),
        |v| Response::default().success("查看文章成功", Some(v)),
    )
}

pub async fn random(
    Json(payload): Json<lc_dto::articles::ArticlePageRequestParams>,
) -> Response<lc_models::articles::ArticleByPage> {
    let article =
        lc_services::articles::article_services::random(payload.page_num, payload.page_size).await;

    article.map_or_else(
        |e| Response::default().fail("查看文章失败", Some(e)),
        |v| Response::default().success("查看文章成功", Some(v)),
    )
}
