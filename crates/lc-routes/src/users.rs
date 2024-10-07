use anyhow::anyhow;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use lc_utils::response::Response;

pub fn build_api_users_router() -> axum::Router {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/login", get(login))
            .route("/register", post(|| async {}))
            .route("/logout", post(|| async {}))
            .route("/profile", post(|| async {}))
            .route("/reset-password", post(|| async {}))
            .route("/reset-nickname", post(|| async {})),
    )
}

async fn login() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .fail("", anyhow!("abc"))
}
