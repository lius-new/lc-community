use anyhow::{anyhow, Result};
use lc_dto::users::{LoginRequestParam, RegisterRequestParam};
use lc_utils::database;

/// 登陆账号
pub async fn login(payload: LoginRequestParam) -> Result<String> {
    let pool = database::get_connection().await?;

    let mut tx = pool.begin().await?;

    let user: lc_models::users::UserInfoWithLogin =
        sqlx::query_as("select password,uuid from user_infos where nickname = $1")
            .bind(&payload.nickname)
            .fetch_one(&mut *tx)
            .await?;

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
        .await?;

    tx.commit().await?; // 提交事务

    if !lc_utils::verify_password(&payload.password.as_bytes(), &user.password) {
        return Err(anyhow!("用户密码错误!"));
    }

    let token_str = lc_utils::sign_with_value(&user.uuid)?;
    Ok(token_str.to_string())
}

/// 注册账号
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
        .bind(6)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

/// 退出登陆
pub async fn logout(uuid: &str) -> Result<bool> {
    let pool = database::get_connection().await?;

    let row = sqlx::query("select id from user_login_infos where uuid = $1;")
        .bind(uuid)
        .execute(pool)
        .await?;

    let sql_str = if row.rows_affected() > 0 {
        "update user_login_infos set login_created_time = null, logout_created_time = now() where uuid = $1;"
    } else {
        "insert into user_login_infos(uuid, logout_created_time) values ($1, now());"
    };

    sqlx::query(sql_str).bind(uuid).execute(pool).await?;

    Ok(true)
}

pub async fn profile(uuid: &str) -> Result<lc_models::users::UserInfoWithProfile> {
    let pool = database::get_connection().await?;

    let user_profile: lc_models::users::UserInfoWithProfile =
        sqlx::query_as("select * from user_infos where uuid = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await?;

    Ok(user_profile)
}

pub async fn reset_password(uuid: &str, new_password: &str) -> Result<()> {
    let pool = database::get_connection().await?;

    let new_password_hash = lc_utils::hash_password(new_password.as_bytes())?;

    sqlx::query("update user_infos set password = $1, updated_at = now() where uuid = $2;")
        .bind(new_password_hash)
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn reset_nickname(uuid: &str, nickname: &str) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update user_infos set nickname = $1, updated_at = now() where uuid = $2;")
        .bind(nickname)
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn reset_gender(uuid: &str, gender: bool) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update user_infos set gender = $1, updated_at = now() where uuid = $2;")
        .bind(gender)
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn reset_phone(uuid: &str, phone: &str) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update user_infos set phone = $1, updated_at = now() where uuid = $2;")
        .bind(phone)
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn reset_email(uuid: &str, email: &str) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update user_infos set email = $1, updated_at = now() where uuid = $2;")
        .bind(email)
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}
