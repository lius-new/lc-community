use axum::{
    extract::{MatchedPath, Request},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use lc_utils::verify_sign_with_token;

#[derive(Clone)]
pub struct CurrentUser {
    pub uuid: String,
}

/// 权限中间件
pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    // 获取token
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 对包含token的字符串进行分割并取第二个元素作为token
    let mut bearer_token = auth_header.split_whitespace();
    bearer_token.next();
    let token_str = if let Some(token_str) = bearer_token.next() {
        token_str
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // 对token进行解析获取包含在token中的用户的uuid字符串。
    let uuid = match verify_sign_with_token(token_str) {
        Ok(data) => data.0,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let resource = req
        .extensions()
        .get::<MatchedPath>()
        .map_or_else(|| req.uri().path(), |path| path.as_str());

    let resource_method = req.method().as_str();

    println!("{}, {}, {}", uuid, resource, resource_method);

    // 进行权限校验，只有用户存在该资源才有权进行访问。
    if let Ok(exist) = lc_services::auth(&uuid, resource, resource_method).await {
        if exist {
            req.extensions_mut().insert(CurrentUser { uuid });
            return Ok(next.run(req).await);
        }
    }

    return Err(StatusCode::UNAUTHORIZED);
}
