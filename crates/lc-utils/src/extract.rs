use axum::{
    async_trait,
    extract::{self, rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// 自定义Extractor
pub struct Json<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for Json<T>
where
    T: DeserializeOwned + Serialize + Debug,
    S: Send + Sync,
{
    type Rejection = crate::response::Response<T>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match extract::Json::<T>::from_request(req, state).await {
            Ok(body) => Ok(Self(body.0)),
            Err(rejection) => {
                let (status, message) = match rejection {
                    JsonRejection::JsonDataError(_) => (StatusCode::BAD_REQUEST, "请求参数错误"),
                    JsonRejection::JsonSyntaxError(_) => {
                        (StatusCode::BAD_REQUEST, "请求参数语法错误")
                    }
                    JsonRejection::MissingJsonContentType(_) => {
                        (StatusCode::BAD_REQUEST, "缺少请求参数")
                    }
                    _ => (StatusCode::OK, ""),
                };

                Err(crate::response::Response::default()
                    .with_status_code(status)
                    .fail(message, Option::None))
            }
        }
    }
}
