use axum::{routing::post, Extension, Router};
use lc_middlewares::auth;

pub fn build_api_articles_router() -> axum::Router {
    Router::new().nest(
        "/articles",
        Router::new().route(
            "/list",
            post(|_state: Extension<auth::CurrentUser>| async move {
                lc_utils::response::Response::default().success("message", Some(()))
            }),
        ),
    )
}
