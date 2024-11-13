use std::collections::HashMap;

use axum::{
    response::Result,
    routing::{get, post},
    Extension, Router,
};
use lc_middlewares::auth;
use lc_utils::{errors::AppError, extract::Json, response::Response};

pub mod api_management {
    use super::*;
    use lc_services::users;

    use lc_dto::users::api_management::{
        LoginRequestParam, RegisterRequestParam, ResetEmailRequestParam, ResetGenderRequestParam,
        ResetNicknameRequestParam, ResetPasswordRequestParam, ResetPhoneRequestParam,
        ViewUserRequestParam,
    };

    pub fn build_router() -> Router {
        Router::new().nest(
            "/users",
            Router::new()
                .route("/login", post(login))
                .route("/create", post(create))
                .route("/logout", get(logout))
                .route("/view", post(view))
                .route("/view-page", post(view_page))
                .route("/reset-nickname", post(reset_nickname))
                .route("/reset-password", post(reset_password))
                .route("/reset-email", post(reset_email))
                .route("/reset-phone", post(repet_phone))
                .route("/reset-gender", post(repet_gender)),
        )
    }

    /// 用户登陆
    async fn login(
        Json(payload): Json<LoginRequestParam>,
    ) -> Result<Response<HashMap<String, String>>, AppError> {
        let token = users::login(&payload.account, &payload.password).await?;
        let mut map = HashMap::new();
        map.insert("token".to_string(), token);

        Ok(Response::default().success("用户登录成功", Some(map)))
    }

    /// 创建用户
    async fn create(Json(payload): Json<RegisterRequestParam>) -> Result<Response<()>, AppError> {
        users::create(&payload.nickname, &payload.password, 0).await?;
        Ok(Response::default().success("创建用户成功!", None))
    }

    /// 用户退出
    async fn logout(state: Extension<auth::CurrentUser>) -> Result<Response<()>, AppError> {
        users::logout(state.uuid.as_ref()).await?;
        Ok(Response::default().success("退出成功", Some(())))
    }

    /// 用户信息查看
    async fn view(
        Json(payload): Json<ViewUserRequestParam>,
    ) -> Result<Response<lc_models::users::UserInfoWithView>, AppError> {
        let user = users::view(&payload.uuid).await?;
        Ok(Response::default().success("获取成功", Some(user)))
    }

    /// 用户信息查看
    async fn view_page(
        Json(payload): Json<lc_dto::PageRequestParams>,
    ) -> Result<Response<(i32, Vec<lc_models::users::UserInfoWithView>)>, AppError> {
        let (total, users) = users::view_page(payload.page_num, payload.page_size).await?;
        Ok(Response::default().success("获取成功", Some((total, users))))
    }

    /// 用户更新密码
    async fn reset_password(
        state: Extension<auth::CurrentUser>,
        Json(payload): Json<ResetPasswordRequestParam>,
    ) -> Result<Response<()>, AppError> {
        users::reset_password(state.uuid.as_str(), payload.password.as_str()).await?;
        Ok(Response::default().success("更新密码成功", Some(())))
    }

    /// 用户更新昵称
    async fn reset_nickname(
        state: Extension<auth::CurrentUser>,
        Json(payload): Json<ResetNicknameRequestParam>,
    ) -> Result<Response<()>, AppError> {
        users::reset_nickname(state.uuid.as_str(), payload.nickname.as_str()).await?;
        Ok(Response::default().success("更新昵称成功", Some(())))
    }

    /// 用户更新邮箱
    async fn reset_email(
        state: Extension<auth::CurrentUser>,
        Json(payload): Json<ResetEmailRequestParam>,
    ) -> Result<Response<()>, AppError> {
        users::reset_email(state.uuid.as_str(), payload.email.as_str()).await?;
        Ok(Response::default().success("更新邮箱成功", Some(())))
    }

    /// 用户更新手机号码
    async fn repet_phone(
        state: Extension<auth::CurrentUser>,
        Json(payload): Json<ResetPhoneRequestParam>,
    ) -> Result<Response<()>, AppError> {
        users::reset_phone(state.uuid.as_str(), payload.phone.as_str()).await?;
        Ok(Response::default().success("更新手机号成功", Some(())))
    }

    /// 用户更新性别
    async fn repet_gender(
        state: Extension<auth::CurrentUser>,
        Json(payload): Json<ResetGenderRequestParam>,
    ) -> Result<Response<()>, AppError> {
        users::reset_gender(state.uuid.as_str(), payload.gender).await?;
        Ok(Response::default().success("更新性别成功", Some(())))
    }
}

pub mod api {

    use super::*;

    pub fn build_router() -> Router {
        Router::new()
    }
}
