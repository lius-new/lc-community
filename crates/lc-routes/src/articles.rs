use axum::{middleware, routing::post, Extension, Router};

pub fn build_api_articles_router() -> axum::Router {
    Router::new()
        .nest(
            "/articles",
            Router::new().route(
                "/list",
                post(
                    |ext: Extension<lc_middlewares::auth::CurrentUser>| async move {
                        lc_utils::response::Response::default().success("message", Some(()))
                    },
                ),
            ),
        )
        .route_layer(middleware::from_fn(lc_middlewares::auth::auth))
}
