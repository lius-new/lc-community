pub mod articles;
pub mod permissions;
pub mod users;

#[tokio::test]
async fn test_auth() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;

    lc_services::auth(
        "e2b868b1-373b-40c6-b6cf-5fa69f66f0f2",
        "/users/register",
        "post",
    )
    .await
    .unwrap();

    assert_eq!(true, true)
}
