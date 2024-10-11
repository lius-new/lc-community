use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use lc_utils::{extract::CustomExtractorJson, response::Response};
use serde::Serialize;

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
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("")
}

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct Register {
    pub nickname: String,
    pub password: String,
}

async fn register(CustomExtractorJson(body): CustomExtractorJson<Register>) -> impl IntoResponse {
    println!("{:?}", body);

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
