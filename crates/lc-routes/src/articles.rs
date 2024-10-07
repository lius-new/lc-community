use axum::{routing::post, Router};

pub fn build_api_articles_router() -> axum::Router {
    Router::new().nest("/articles", Router::new().route("/list", post(|| async {})))
}
