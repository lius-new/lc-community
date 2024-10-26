use lc_utils::response::Response;

pub async fn view_by_hash() -> Response<()> {
    let article = lc_services::articles::article_services::view_by_hash("abc").await;

    article.map_or_else(
        |e| Response::default().fail("获取文章失败", Some(e)),
        |v| Response::default().success("获取文章成功", Some(v)),
    )
}

pub async fn create() -> Response<()> {
    let article = lc_services::articles::article_services::create().await;

    article.map_or_else(
        |e| Response::default().fail("创建文章失败", Some(e)),
        |v| Response::default().success("创建文章成功", Some(v)),
    )
}

pub async fn modify() -> Response<()> {
    let article = lc_services::articles::article_services::modify().await;

    article.map_or_else(
        |e| Response::default().fail("修改文章失败", Some(e)),
        |v| Response::default().success("修改文章成功", Some(v)),
    )
}

pub async fn delete_by_hash() -> Response<()> {
    let article = lc_services::articles::article_services::delete_by_hash().await;

    article.map_or_else(
        |e| Response::default().fail("删除文章失败", Some(e)),
        |v| Response::default().success("删除文章成功", Some(v)),
    )
}

pub async fn toggle_visiable() -> Response<()> {
    let article = lc_services::articles::article_services::toggle_visiable().await;

    article.map_or_else(
        |e| Response::default().fail("更改文章可见性失败", Some(e)),
        |v| Response::default().success("删除文章可见性成功", Some(v)),
    )
}

pub async fn page() -> Response<()> {
    let article = lc_services::articles::article_services::page().await;

    article.map_or_else(
        |e| Response::default().fail("查看文章失败", Some(e)),
        |v| Response::default().success("查看文章成功", Some(v)),
    )
}

pub async fn toplist() -> Response<()> {
    let article = lc_services::articles::article_services::toplist().await;

    article.map_or_else(
        |e| Response::default().fail("查看文章失败", Some(e)),
        |v| Response::default().success("查看文章成功", Some(v)),
    )
}

pub async fn random() -> Response<()> {
    let article = lc_services::articles::article_services::random().await;

    article.map_or_else(
        |e| Response::default().fail("查看文章失败", Some(e)),
        |v| Response::default().success("查看文章成功", Some(v)),
    )
}
