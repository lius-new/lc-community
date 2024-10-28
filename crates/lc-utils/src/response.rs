use anyhow::{anyhow, Error};
use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use serde_json::json;
use std::fmt::{Debug, Display};

const SUCCESS_CODE: i32 = 1;
const WARN_CODE: i32 = 1;
const FAIL_CODE: i32 = -1;

/// 统一响应类型
#[derive(Debug, serde::Serialize, Clone)]
pub struct Response<T> {
    pub status_code: u16,
    pub code: i32,
    pub data: Option<T>,
    pub message: String,
}

impl<T> Response<T>
where
    T: Debug,
{
    /// 初始化默认响应类型
    pub fn default() -> Self {
        Response {
            status_code: 200,
            code: WARN_CODE,
            data: None,
            message: "".to_string(),
        }
    }
    /// 操作成功对应的响应类型
    pub fn success(mut self, message: &str, data: Option<T>) -> Self {
        self.message = message.to_string();
        self.data = data;
        self.code = SUCCESS_CODE;
        self
    }
    /// 操作失败对应的响应类型
    pub fn fail(mut self, message: &str, err: Option<Error>) -> Self {
        let err = err.map_or("".to_string(), |e| e.to_string());

        self.message = if !err.is_empty() {
            if message.is_empty() {
                format!("{}", err)
            } else {
                format!("{}: {}", message, err)
            }
        } else {
            message.to_string()
        };

        self.code = FAIL_CODE;
        self
    }

    /// 指定响应的状态码，如果不指定就会使用默认的(200)
    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code.into();
        self
    }
    /// 指定响应数据，如果不指定则该数据字段不会出现在json中。
    pub fn with_data(mut self, data: Option<T>) -> Self {
        self.data = data;
        self
    }
    /// 指定响应信息。
    pub fn with_message(mut self, message: &str) -> Self {
        self.message += message;
        self
    }
}

// 允许直接打印和to_string
impl<T> Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// 当实现该类型后可以直接将Response<T>作为axum路由处理函数的返回值，会自动调用该trait的into_response方法最终返回json.
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

        match StatusCode::from_u16(self.status_code) {
            Ok(s) => (s, Json(content_json)).into_response(),
            Err(_) => (StatusCode::BAD_REQUEST, Json(content_json)).into_response(),
        }
    }
}

/// 实现From<MultipartError>
impl From<MultipartError> for Response<()> {
    fn from(value: MultipartError) -> Self {
        Response::default()
            .with_status_code(value.status())
            .fail("param not found", Some(anyhow!(value.body_text())))
    }
}
