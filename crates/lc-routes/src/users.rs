use std::collections::HashMap;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use lc_dto::users::{
    LoginRequestParam, RegisterRequestParam, ResetEmailRequestParam, ResetGenderRequestParam,
    ResetNicknameRequestParam, ResetPasswordRequestParam, ResetPhoneRequestParam,
};
use lc_middlewares::auth;
use lc_utils::{extract::Json, response::Response};

pub mod api_management {
    use super::*;

    pub fn build_router() -> Router {
        Router::new().nest(
            "/users",
            Router::new()
                .route("/login", post(login))
                .route("/register", post(register))
                .route("/logout", get(logout))
                .route("/profile", get(profile))
                .route("/reset-nickname", post(reset_nickname))
                .route("/reset-password", post(reset_password))
                .route("/reset-email", post(reset_email))
                .route("/reset-phone", post(repet_phone))
                .route("/reset-gender", post(repet_gender)),
        )
    }
}
pub mod api {

    use super::*;

    pub fn build_router() -> Router {
        Router::new()
    }
}

/// 用户登陆
async fn login(Json(payload): Json<LoginRequestParam>) -> Response<HashMap<String, String>> {
    lc_services::users::login(payload).await.map_or_else(
        |e| Response::default().fail("用户登陆失败", Some(e)),
        |v| {
            let mut map = HashMap::new();
            map.insert("token".to_string(), v);
            Response::default().success("用户登陆成功", Some(map))
        },
    )
}

/// 用户注册
async fn register(Json(payload): Json<RegisterRequestParam>) -> Response<()> {
    lc_services::users::register(payload).await.map_or_else(
        |e| Response::default().fail("用户注册失败", Some(e)),
        |_| Response::default().success("用户注册成功", None),
    )
}

/// 用户退出
async fn logout(state: Extension<auth::CurrentUser>) -> Response<()> {
    lc_services::users::logout(state.uuid.as_ref())
        .await
        .map_or_else(
            |e| Response::default().fail("退出失败", Some(e)),
            |v| {
                if v {
                    Response::default().success("退出成功", Some(()))
                } else {
                    Response::default().success("退出成功", Some(()))
                }
            },
        )
}

/// 用户信息查看
async fn profile(
    state: Extension<auth::CurrentUser>,
) -> Response<lc_models::users::UserInfoWithProfile> {
    lc_services::users::profile(state.uuid.as_str())
        .await
        .map_or_else(
            |e| Response::default().fail("获取失败", Some(e)),
            |v| Response::default().success("获取成功", Some(v)),
        )
}

/// 用户更新密码
async fn reset_password(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetPasswordRequestParam>,
) -> Response<()> {
    lc_services::users::reset_password(state.uuid.as_str(), payload.password.as_str())
        .await
        .map_or_else(
            |e| Response::default().fail("更新密码失败", Some(e)),
            |_| Response::default().success("更新密码成功", Some(())),
        )
}

/// 用户更新昵称
async fn reset_nickname(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetNicknameRequestParam>,
) -> Response<()> {
    lc_services::users::reset_nickname(state.uuid.as_str(), payload.nickname.as_str())
        .await
        .map_or_else(
            |e| Response::default().fail("更新昵称失败", Some(e)),
            |_| Response::default().success("更新密码成功", Some(())),
        )
}

/// 用户更新邮箱
async fn reset_email(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetEmailRequestParam>,
) -> Response<()> {
    lc_services::users::reset_email(state.uuid.as_str(), payload.email.as_str())
        .await
        .map_or_else(
            |e| Response::default().fail("更新邮箱失败", Some(e)),
            |_| Response::default().success("更新邮箱成功", Some(())),
        )
}

/// 用户更新手机号码
async fn repet_phone(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetPhoneRequestParam>,
) -> Response<()> {
    lc_services::users::reset_phone(state.uuid.as_str(), payload.phone.as_str())
        .await
        .map_or_else(
            |e| Response::default().fail("更新手机号失败", Some(e)),
            |_| Response::default().success("更新手机号成功", Some(())),
        )
}

/// 用户更新性别
async fn repet_gender(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetGenderRequestParam>,
) -> Response<()> {
    lc_services::users::reset_gender(state.uuid.as_str(), payload.gender)
        .await
        .map_or_else(
            |e| Response::default().fail("更新性别失败", Some(e)),
            |_| Response::default().success("更新性别成功", Some(())),
        )
}
