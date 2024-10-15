use anyhow::{Ok, Result};
use lc_dto::users::{LoginRequestParam, RegisterRequestParam};
use lc_utils::database;

pub async fn login(payload: LoginRequestParam) -> Result<LoginRequestParam> {
    let pool = database::get_connection().await?;

    let row: (i64,) = sqlx::query_as("SELECT ")
        .bind(150_i64)
        .fetch_one(pool)
        .await?;

    println!("{:?}", row);

    Ok(payload)
}

pub async fn register(payload: RegisterRequestParam) -> Result<()> {
    let pool = database::get_connection().await?;

    let mut tx = pool.begin().await?;

    let uuid = lc_utils::uuid();
    let password = lc_utils::hash_password(payload.password.as_bytes())?;

    sqlx::query("INSERT INTO users(uuid) VALUES ($1);")
        .bind(&uuid)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO user_infos(nickname, password, uuid) VALUES ($1, $2, $3)")
        .bind(&payload.nickname)
        .bind(password)
        .bind(&uuid)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO user_group_relations(uuid, user_group_id) VALUES ($1, $2);")
        .bind(&uuid)
        .bind(1)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
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
