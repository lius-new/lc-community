use axum::{http::StatusCode, routing::post, Router};
use lc_dto::users::{LoginRequestParam, RegisterRequestParam};
use lc_utils::{extract::Json, response::Response};

pub fn build_api_users_router() -> axum::Router {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/login", post(login))
            .route("/register", post(register))
            .route("/logout", post(logout))
            .route("/profile", post(profile))
            .route("/reset-password", post(reset_password))
            .route("/reset-nickname", post(reset_nickname)),
    )
}

async fn login(Json(payload): Json<LoginRequestParam>) -> Response<LoginRequestParam> {
    lc_services::users::login(payload).await.into()
}

async fn register(Json(payload): Json<RegisterRequestParam>) -> Response<()> {
    println!("{:?}", payload);

    Response::<()>::default()
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
