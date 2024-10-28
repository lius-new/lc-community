use axum::{
    routing::{get, post},
    Router,
};
pub mod api_management {
    use super::*;
    pub fn build_router() -> Router {
        Router::new().nest(
            "/articles",
            Router::new()
                .nest(
                    "/",
                    Router::new()
                        .route("/:hash", post(article_handle::view_by_hash))
                        .route("/create", post(article_handle::create))
                        .route("/modify", post(article_handle::modify))
                        .route("/delete", post(article_handle::delete_by_hash))
                        .route("/toggle-visiable", post(article_handle::toggle_visiable))
                        .route("/page", post(article_handle::page))
                        .route("/toplist", post(article_handle::toplist))
                        .route("/random", post(article_handle::random)),
                )
                .nest(
                    "/group",
                    Router::new()
                        .route("/create", get(group_handle::create))
                        .route("/modify", get(group_handle::modify))
                        .route("/delete", post(group_handle::delete))
                        .route("/toggle-visiable", get(group_handle::toggle_visiable)),
                )
                .nest(
                    "/tag",
                    Router::new()
                        .route("/create", get(tag_handle::create))
                        .route("/modify", get(tag_handle::modify))
                        .route("/delete", post(tag_handle::delete))
                        .route("/toggle-visiable", get(tag_handle::toggle_visiable)),
                ),
        )
    }
}

pub mod api {
    use super::*;
    pub fn build_router() -> Router {
        Router::new()
    }
}

mod article_handle;
mod group_handle;
mod tag_handle;
