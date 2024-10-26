use axum::{http::StatusCode, middleware, Router};

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}

pub fn build_root_router() -> Router {
    Router::new()
        .nest(
            "/api/management/v1",
            Router::new()
                .merge(users::api_management::build_router())
                .merge(articles::api_management::build_router())
                .merge(permissions::api_management::build_router()),
        )
        .nest(
            "/api/v1",
            Router::new()
                .merge(articles::api::build_router())
                .merge(users::api::build_router())
                .merge(permissions::api::build_router()),
        )
        .route_layer(middleware::from_fn(lc_middlewares::auth::auth))
        .fallback(not_found)
}

pub mod articles;
pub mod permissions;
pub mod users;
