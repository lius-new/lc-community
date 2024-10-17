use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use lc_utils::verify_sign_with_token;

pub async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut bearer_token = auth_header.split_whitespace();
    bearer_token.next();
    let token_str = if let Some(token_str) = bearer_token.next() {
        token_str
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let data = match verify_sign_with_token(token_str) {
        Ok(data) => data.0,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    println!("{}", data);

    Ok(next.run(req).await)
}
