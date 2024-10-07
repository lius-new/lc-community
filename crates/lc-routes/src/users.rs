use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use lc_utils::response::Response;
use sqlx;

pub fn build_api_users_router() -> axum::Router {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/login", get(login))
            .route("/register", post(register))
            .route("/logout", post(logout))
            .route("/profile", post(profile))
            .route("/reset-password", post(reset_password))
            .route("/reset-nickname", post(reset_nickname)),
    )
}

async fn login() -> Response<()> {
    let pool = lc_utils::database::DB.get().unwrap().get().await;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool)
        .await
        .unwrap();
    println!("{:?}", row);

    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("")
}
async fn register() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("")
}
async fn logout() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("")
}
async fn profile() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("")
}
async fn reset_password() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("")
}
async fn reset_nickname() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("")
}
