use anyhow::{anyhow, Context, Ok, Result};
use lc_dto::users::{LoginRequestParam, RegisterRequestParam};
use lc_utils::{database, verify_password};

pub async fn login(payload: LoginRequestParam) -> Result<String> {
    let pool = database::get_connection().await?;

    let user: lc_models::users::UserInfoWithLogin =
        sqlx::query_as("select nickname,password from user_infos where nickname = $1")
            .bind(&payload.nickname)
            .fetch_one(pool)
            .await
            .context("用户不存在")?;

    if !verify_password(&payload.password.as_bytes(), &user.password) {
        return Err(anyhow!("用户密码错误!"));
    }

    Ok("token ...".to_string())
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
        .await
        .context("用户已经存在")?;

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
