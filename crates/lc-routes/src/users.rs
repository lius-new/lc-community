use axum::{http::StatusCode, routing::post, Router};
use lc_dto::users::{LoginRequestParam, RegisterRequestParam};
use lc_utils::{extract::Json, response::Response};
use tokio::time::Instant;

pub fn build_api_users_router() -> axum::Router {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/login", post(login))
            .route("/register", post(register))
            .route("/logout", post(logout))
            .route("/profile", post(profile))
            .route("/reset-password", post(reset_password))
            .route("/reset-nickname", post(reset_nickname)),
    )
}

async fn login(Json(payload): Json<LoginRequestParam>) -> Response<String> {
    match lc_services::users::login(payload).await {
        Ok(s) => Response::default().success("用户登陆成功", Some(s)),
        Err(e) => Response::default().fail("用户登陆失败", Some(e)),
    }
}

async fn register(Json(payload): Json<RegisterRequestParam>) -> Response<()> {
    let now = Instant::now();
    let r = lc_services::users::register(payload).await;

    println!("register elapsed : ({:?})", now.elapsed());

    match r {
        Ok(_) => Response::default().success("用户注册成功", None),
        Err(e) => Response::default().fail("用户注册失败", Some(e)),
    }
}
async fn logout() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("", None)
}
async fn profile() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("", None)
}
async fn reset_password() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("", None)
}
async fn reset_nickname() -> Response<()> {
    Response::default()
        .with_status_code(StatusCode::BAD_REQUEST)
        .success("", None)
}
