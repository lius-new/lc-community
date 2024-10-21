use anyhow::{anyhow, Context, Ok, Result};
use lc_dto::users::{LoginRequestParam, RegisterRequestParam};
use lc_utils::database;
use tokio::time::Instant;

pub async fn login(payload: LoginRequestParam) -> Result<String> {
    let pool = database::get_connection().await?;

    let mut tx = pool.begin().await?;

    let user: lc_models::users::UserInfoWithLogin =
        sqlx::query_as("select password,uuid from user_infos where nickname = $1")
            .bind(&payload.nickname)
            .fetch_one(&mut *tx)
            .await
            .context("用户不存在")?;

    let row = sqlx::query("select id from user_login_infos where uuid = $1;")
        .bind(&user.uuid)
        .execute(&mut *tx)
        .await?;

    let sql_str = if row.rows_affected() > 0 {
        "update user_login_infos set login_created_time = now() where uuid = $1;"
    } else {
        "insert into user_login_infos(uuid)  values ($1);"
    };

    sqlx::query(sql_str)
        .bind(&user.uuid)
        .execute(&mut *tx)
        .await
        .context("用户不存在")?;

    tx.commit().await?; // 提交事务

    // TODO: instant 区域代码执行慢。
    let now = Instant::now();
    if !lc_utils::verify_password(&payload.password.as_bytes(), &user.password) {
        return Err(anyhow!("用户密码错误!"));
    }

    let token_str = lc_utils::sign_with_value(&user.uuid)?;
    println!("{:?}", now.elapsed());
    Ok(token_str.to_string())
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
