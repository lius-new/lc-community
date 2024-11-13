use axum::http::{
    header::{self},
    HeaderName, Method,
};
use tower_http::cors::CorsLayer;

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers([
            header::CONTENT_TYPE,
            HeaderName::from_static("x-custom-header"),
        ])
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // allow requests from any origin
        .allow_origin(["http://localhost:5173".parse().unwrap()])
}
