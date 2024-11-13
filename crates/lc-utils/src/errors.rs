use anyhow::anyhow;
use axum::{http::StatusCode, response::IntoResponse};
use std::fmt::Debug;
use thiserror::Error;

use crate::response;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("database failed: {0}")]
    DatabaseError(DatabaseError),

    #[error("request failed: {0}")]
    RequestError(RequestError),

    #[error("{0}")]
    CustomerError(String),

    #[error("未知错误")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("request param not found: {0}")]
    ParamNotFoundError(String),

    #[error("request matlipart param failed: {0}")]
    MatlipartParseError(String),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(" failed to get database connection! ")]
    ConnectionFailed,

    #[error("database failed: {0}")]
    SqlxError(sqlx::Error),
}

impl AppError {
    /// 获取错误对应的http响应状态码
    /// 默认都是StatusCode::OK: 200
    pub fn status_code(&self) -> StatusCode {
        match self {
            _ => StatusCode::OK,
        }
    }

    /// 获取错误对应描述信息
    pub fn error(&self) -> String {
        match self {
            _ => self.to_string(),
        }
    }
}

/// 实现从sqlx::Error转换AppError
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            _ => AppError::DatabaseError(DatabaseError::SqlxError(error)),
        }
    }
}

/// 实现从serde_json::Error转换AppError
impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::CustomerError(format!("{:?}", error))
    }
}

/// 实现从anyhow::Error转换AppError
impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::CustomerError(format!("{:?}", error))
    }
}

/// 实现将AppError转换为IntoResponse
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        response::Response::<()>::default()
            .fail("", Some(anyhow!(self.to_string())))
            .into_response()
    }
}

pub mod result {
    use super::AppError;

    // 该Result可以转换实现了impl axum::Response的response::Response
    // 当提供(String,T)时，String对应Response.的message, T对应Response.data
    pub type Result<T, E = AppError> = std::result::Result<T, E>;
}
