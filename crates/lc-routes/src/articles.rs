use axum::{routing::post, Extension, Router};

pub fn build_api_articles_router() -> axum::Router {
    Router::new().nest(
        "/articles",
        Router::new().route(
            "/list",
            post(
                |_ext: Extension<lc_middlewares::auth::CurrentUser>| async move {
                    lc_utils::response::Response::default().success("message", Some(()))
                },
            ),
        ),
    )
}
