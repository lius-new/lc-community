pub async fn setup() {
    let (url, max_connections) = ("postgres://lius:lsmima@127.0.0.1/lcdb", 5);
    lc_utils::database::init_db(url, max_connections).await;
}
