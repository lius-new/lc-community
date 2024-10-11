use axum::{
    async_trait,
    extract::{FromRequest, Json, Request},
    http::StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// 自定义Extractor
pub struct CustomExtractorJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for CustomExtractorJson<T>
where
    T: DeserializeOwned + Serialize + Debug,
    S: Send + Sync,
{
    type Rejection = crate::response::Response<T>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(body) => Ok(Self(body.0)),
            Err(e) => Err(crate::response::Response::default()
                .with_status_code(StatusCode::BAD_REQUEST)
                .fail("", e.into())),
        }
    }
}
