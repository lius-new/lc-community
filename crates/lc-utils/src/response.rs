use anyhow::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use serde_json::json;
use std::fmt::{Debug, Display};

const SUCCESS_CODE: i32 = 1;
const WARN_CODE: i32 = 1;
const FAIL_CODE: i32 = -1;

pub struct Response<T> {
    pub status_code: StatusCode,
    pub code: i32,
    pub data: Option<T>,
    pub message: String,
}

impl<T> Response<T>
where
    T: Debug + Serialize,
{
    pub fn default() -> Self {
        Response {
            status_code: StatusCode::OK,
            code: WARN_CODE,
            data: None,
            message: "".to_string(),
        }
    }
    pub fn success(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self.code = SUCCESS_CODE;
        self
    }

    pub fn fail(mut self, message: &str, err: Error) -> Self {
        if self.message.len() > 0 {
            self.message = format!("{}: {}", message, err.to_string());
        } else {
            self.message = err.to_string()
        }
        self.code = FAIL_CODE;
        self
    }

    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn with_data(mut self, data: Option<T>) -> Self {
        self.data = data;
        self
    }
    pub fn with_message(mut self, message: &str) -> Self {
        self.message += message;
        self
    }
}

impl<T> Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T> IntoResponse for Response<T>
where
    T: Debug + Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let mut content_json = json!({ "code": self.code,"message": self.message.to_string() });

        if let Some(data) = &self.data {
            content_json =
                json!({ "code": self.code,"message": self.message.to_string(),"data": data});
        }

        (self.status_code, Json(content_json)).into_response()
    }
}
