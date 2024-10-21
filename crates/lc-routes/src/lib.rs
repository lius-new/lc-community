use axum::{http::StatusCode, middleware, Router};

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}
pub fn build_api_root_router() -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(users::build_api_users_router())
                .merge(articles::build_api_articles_router()),
        )
        .route_layer(middleware::from_fn(lc_middlewares::auth::auth))
        .fallback(not_found)
}

pub mod articles;
pub mod users;
