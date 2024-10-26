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

pub fn build_api_users_router() -> axum::Router {
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

async fn login(Json(payload): Json<LoginRequestParam>) -> Response<HashMap<String, String>> {
    match lc_services::users::login(payload).await {
        Ok(s) => {
            let mut map = HashMap::new();
            map.insert("token".to_string(), s);
            Response::default().success("用户登陆成功", Some(map))
        }
        Err(e) => Response::default().fail("用户登陆失败", Some(e)),
    }
}

async fn register(Json(payload): Json<RegisterRequestParam>) -> Response<()> {
    let r = lc_services::users::register(payload).await;

    match r {
        Ok(_) => Response::default().success("用户注册成功", None),
        Err(e) => Response::default().fail("用户注册失败", Some(e)),
    }
}

async fn logout(state: Extension<auth::CurrentUser>) -> Response<()> {
    let r = lc_services::users::logout(state.uuid.as_ref()).await;

    let response = Response::default();
    match r {
        Ok(v) => {
            if v {
                return response.success("退出成功", Some(()));
            }
            return response.success("退出成功", Some(()));
        }
        Err(e) => return response.fail("退出失败", Some(e)),
    }
}

async fn profile(
    state: Extension<auth::CurrentUser>,
) -> Response<lc_models::users::UserInfoWithProfile> {
    let response = Response::default();
    let result = lc_services::users::profile(state.uuid.as_str()).await;
    match result {
        Ok(v) => response.success("获取成功", Some(v)),
        Err(e) => return response.fail("获取失败", Some(e)),
    }
}

async fn reset_password(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetPasswordRequestParam>,
) -> Response<()> {
    let response = Response::default();
    let result =
        lc_services::users::reset_password(state.uuid.as_str(), payload.password.as_str()).await;
    match result {
        Ok(_) => response.success("更新密码成功", Some(())),
        Err(e) => return response.fail("更新密码失败", Some(e)),
    }
}

async fn reset_nickname(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetNicknameRequestParam>,
) -> Response<()> {
    let response = Response::default();
    let result =
        lc_services::users::reset_nickname(state.uuid.as_str(), payload.nickname.as_str()).await;
    match result {
        Ok(_) => response.success("更新昵称成功", Some(())),
        Err(e) => return response.fail("更新昵称失败", Some(e)),
    }
}

async fn reset_email(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetEmailRequestParam>,
) -> Response<()> {
    let response = Response::default();
    let result = lc_services::users::reset_email(state.uuid.as_str(), payload.email.as_str()).await;
    match result {
        Ok(_) => response.success("更新邮箱成功", Some(())),
        Err(e) => return response.fail("更新邮箱失败", Some(e)),
    }
}

async fn repet_phone(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetPhoneRequestParam>,
) -> Response<()> {
    let response = Response::default();
    let result = lc_services::users::reset_phone(state.uuid.as_str(), payload.phone.as_str()).await;
    match result {
        Ok(_) => response.success("更新手机号成功", Some(())),
        Err(e) => return response.fail("更新手机号失败", Some(e)),
    }
}

async fn repet_gender(
    state: Extension<auth::CurrentUser>,
    Json(payload): Json<ResetGenderRequestParam>,
) -> Response<()> {
    let response = Response::default();
    let result = lc_services::users::reset_gender(state.uuid.as_str(), payload.gender).await;
    match result {
        Ok(_) => response.success("更新性别成功", Some(())),
        Err(e) => return response.fail("更新性别失败", Some(e)),
    }
}
