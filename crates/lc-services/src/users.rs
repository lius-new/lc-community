use anyhow::{Ok, Result};
use lc_dto::users::LoginRequestParam;
use lc_utils::database;

pub async fn login(payload: LoginRequestParam) -> Result<LoginRequestParam> {
    let pool = database::get_connection().await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool)
        .await?;

    println!("{:?}", row);

    Ok(payload)
}
pub async fn register() -> Result<()> {
    todo!()
}
pub async fn logout() -> Result<()> {
    todo!()
}
pub async fn profile() -> Result<()> {
    todo!()
}
pub async fn reset_password() -> Result<()> {
    todo!()
}
pub async fn reset_nickname() -> Result<()> {
    todo!()
}
