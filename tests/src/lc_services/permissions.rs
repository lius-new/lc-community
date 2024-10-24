use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// 测试显示数据库中的资源表
#[tokio::test]
async fn test_show_all_resource_type() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let types = lc_services::permissions::show_all_resource_type()
        .await
        .unwrap();

    println!("{:?}", types);
}

/// 测试显示数据库中的资源权限关联表
#[tokio::test]
async fn test_show_all_rp_type() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let types = lc_services::permissions::show_all_rp_type().await.unwrap();

    println!("{:?}", types);
}

/// 测试显示所有的资源以及资源对应的权限.
#[tokio::test]
async fn test_show_allresource_permissions() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let alls = lc_services::permissions::show_allresource_permissions(None, None)
        .await
        .unwrap();

    alls.iter().for_each(|v| {
        println!("{:?}", v);
        println!("");
    });
}

/// 测试 添加资源(该资源表示接口资源，用作权限系统)
#[tokio::test]
async fn test_push_resources() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let _ =
        lc_services::permissions::push_resources(lc_dto::permissions::PushResourcesRequestParam {
            resource_name: "".to_string(),
            resource_description: "String".to_string(),
            resource: "String".to_string(),
            resource_method: "String".to_string(),
            resource_table: "String".to_string(),
        })
        .await
        .unwrap();
}

/// 测试资源是否允许访问
#[tokio::test]
async fn test_toggle_canuse_resources() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let _ = lc_services::permissions::toggle_canuse_resources(1, "white_resources")
        .await
        .unwrap();
}

/// 测试资源是否允许访问
#[tokio::test]
async fn test_grant_permissions_with_resources() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let permission_id = vec![2];
    let permission_ids = vec![3, 4, 5];

    let _ = lc_services::permissions::grant_permissions_with_resources(
        3,
        "permission_user_resource_relations",
        permission_ids,
    )
    .await
    .unwrap();
}

/// 测试移除资源权限
#[tokio::test]
async fn test_remove_permissions_with_resources() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let permission_id = vec![2];
    let permission_ids = vec![3, 4, 5];

    let _ = lc_services::permissions::remove_permissions_with_resources(
        3,
        "permission_user_resource_relations",
        permission_ids,
    )
    .await
    .unwrap();
}

/// 测试显示所有权限信息
#[tokio::test]
async fn test_show_all_permissions() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let result = lc_services::permissions::show_all_permissions()
        .await
        .unwrap();

    println!("{:?}", result)
}

/// 测试 显示指定资源权限
#[tokio::test]
async fn test_show_permissions_with_current_resources() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let result =
        lc_services::permissions::show_permissions_with_current_resources("GET+/api/users/logout")
            .await
            .unwrap();

    println!("{:?}", result)
}

/// 测试 根据权限来筛选 显示指定资源权限
#[tokio::test]
async fn test_show_permissions_with_current_permissions() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    let result = lc_services::permissions::show_permissions_with_current_permissions(1)
        .await
        .unwrap();

    println!("{:?}", result)
}
