use axum::{middleware, routing::post, Router};

pub fn build_api_articles_router() -> axum::Router {
    Router::new()
        .nest(
            "/articles",
            Router::new().route(
                "/list",
                post(|| async {
                    lc_utils::response::Response::default().success("message", Some(()))
                }),
            ),
        )
        .route_layer(middleware::from_fn(lc_middlewares::auth))
}
