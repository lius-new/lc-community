use lc_utils::response::Response;

pub async fn create() -> Response<()> {
    let article = lc_services::articles::article_services::view_by_hash("abc").await;

    article.map_or_else(
        |e| Response::default().fail("获取文章失败", Some(e)),
        |v| Response::default().success("获取文章成功", Some(v)),
    )
}

pub async fn modify() -> Response<()> {
    let article = lc_services::articles::article_services::view_by_hash("abc").await;

    article.map_or_else(
        |e| Response::default().fail("获取文章失败", Some(e)),
        |v| Response::default().success("获取文章成功", Some(v)),
    )
}

pub async fn delete() -> Response<()> {
    let article = lc_services::articles::article_services::view_by_hash("abc").await;

    article.map_or_else(
        |e| Response::default().fail("获取文章失败", Some(e)),
        |v| Response::default().success("获取文章成功", Some(v)),
    )
}

pub async fn toggle_visiable() -> Response<()> {
    let article = lc_services::articles::article_services::view_by_hash("abc").await;

    article.map_or_else(
        |e| Response::default().fail("获取文章失败", Some(e)),
        |v| Response::default().success("获取文章成功", Some(v)),
    )
}
