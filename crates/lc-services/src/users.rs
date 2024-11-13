use anyhow::{anyhow, Context, Result};
use lc_models::users::UserInfoWithView;
use lc_utils::database;

/// 创建账号并指定账号的权限
///
/// 生成uuid和密码，并添加到users、user_infos、user_group_relations表中。
///
/// params:
/// - nickname: 用户名
/// - password: 密码
/// - permission_id: 权限id
///
/// return: ()
pub async fn create(nickname: &str, password: &str, group_id: i32) -> Result<()> {
    let pool = database::get_connection().await?;

    let mut tx = pool.begin().await?;

    let uuid = lc_utils::uuid();
    let password = lc_utils::hash_password(password.as_bytes())?;

    sqlx::query("INSERT INTO users(uuid) VALUES ($1);")
        .bind(&uuid)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO user_infos(nickname, password, uuid) VALUES ($1, $2, $3)")
        .bind(nickname)
        .bind(password)
        .bind(&uuid)
        .execute(&mut *tx)
        .await
        .map_err(|_| anyhow!("用户注册失败：用户昵称已存在!"))?;

    // 默认为6
    let group_id = if group_id == 0 { 6 } else { group_id };
    sqlx::query("INSERT INTO user_group_relations(uuid, user_group_id) VALUES ($1, $2);")
        .bind(&uuid)
        .bind(group_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

/// 登陆账号
///
/// 根据用户名查询密码和uuid, 并修改user_login_infos表中该用户的登陆信息。
///
/// params:
/// - nickname: 用户名
/// - password: 密码
/// - permission_id: 权限id
///
/// return: token
pub async fn login(account: &str, password: &str) -> lc_utils::errors::result::Result<String> {
    let pool = database::get_connection().await?;
    let mut tx = pool.begin().await?;

    // 获取用户信息
    let user: lc_models::users::UserInfoWithLogin =
        sqlx::query_as("select password,uuid from user_infos where nickname = $1")
            .bind(account)
            .fetch_one(&mut *tx)
            .await
            .context("用户登录失败：用户不存在")?;

    // 获取用户登陆信息
    let row = sqlx::query("select id from user_login_infos where uuid = $1;")
        .bind(&user.uuid)
        .execute(&mut *tx)
        .await?;

    // 判断是否存在登陆信息，如果存在就更新，如果不存在就创建。
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

    // 校验密码是否正常
    if !lc_utils::verify_password(password.as_bytes(), &user.password) {
        return Err(anyhow!("用户密码错误!").into());
    }

    // 生成token
    let token_str = lc_utils::sign_with_value(&user.uuid)?;
    Ok(token_str.to_string())
}

/// 退出登陆
///
/// 修改user_login_infos表使其处于推出登陆状态
///
/// params:
/// - uuid: 用户uuid
///
/// return: bool(是否退出成功)
pub async fn logout(uuid: &str) -> Result<bool> {
    let pool = database::get_connection().await?;

    // 查询用户登陆信息
    let row = sqlx::query("select id from user_login_infos where uuid = $1;")
        .bind(uuid)
        .execute(pool)
        .await?;

    // 根据登陆信息是否存在来决定更新还是创建
    let sql_str = if row.rows_affected() > 0 {
        "update user_login_infos set login_created_time = null, logout_created_time = now() where uuid = $1;"
    } else {
        "insert into user_login_infos(uuid, logout_created_time) values ($1, now());"
    };

    sqlx::query(sql_str).bind(uuid).execute(pool).await?;

    Ok(true)
}

/// 获取用户信息
///
/// 根据uuid查询user_infos表来获取用户信息
///
/// params:
/// - uuid: 用户uuid
///
/// return: lc_models::users::UserInfoWithProfile(个人信息)
pub async fn view(uuid: &str) -> Result<UserInfoWithView> {
    let pool = database::get_connection().await?;

    let user_profile: UserInfoWithView = sqlx::query_as(
        "
            select
              ui.nickname,
              ui.gender,
              ui.email,
              ui.phone,
              ugr_ug.name,
              ugr_ug.description
            from
              user_infos ui
            left outer join (
              select
                ug.name,
                ug.description,
                ugr.uuid
              from
                user_group_relations ugr
              left outer join user_groups ug on
                ugr.user_group_id = ug.id
            ) ugr_ug on
              ugr_ug.uuid = ui.uuid
            where ui.uuid = $1;",
    )
    .bind(uuid)
    .fetch_one(pool)
    .await?;

    Ok(user_profile)
}

/// 分页获取用户信息
///
/// 根据uuid查询user_infos表来获取用户信息
///
/// params:
/// - page_num: i32,
/// - page_size: i32,
///
/// return: lc_models::users::UserInfoWithProfile(个人信息)
pub async fn view_page(mut page_num: i32, page_size: i32) -> Result<(i32, Vec<UserInfoWithView>)> {
    let pool = database::get_connection().await?;

    let (total,): (i64,) = sqlx::query_as("select count(0) from users;")
        .fetch_one(pool)
        .await?;

    // 判断page_num是否小于0或超过总数, 如果成立则设置为0
    if page_num <= 0 || ((page_num * page_size) as i64) > total {
        page_num = 0
    };

    let offset = page_num * page_size;

    // 执行分页查询
    let users: Vec<UserInfoWithView> = sqlx::query_as(
        "
            select
              ui.nickname,
              ui.gender,
              ui.email,
              ui.phone,
              ugr_ug.name,
              ugr_ug.description
            from
              user_infos ui
            left outer join (
              select
                ug.name,
                ug.description,
                ugr.uuid
              from
                user_group_relations ugr
              left outer join user_groups ug on
                ugr.user_group_id = ug.id
            ) ugr_ug on
              ugr_ug.uuid = ui.uuid
            order by
              ui.created_at,
              ui.updated_at
            limit $1 offset $2;",
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((1, users))
}

/// 更新用户密码
///
/// 生成一个新的密码并1️⃣uuid为条件更新到user_infos的密码字段
///
/// params:
/// - uuid: 用户uuid
/// - new_password: 新密码
///
/// return: ()
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

/// 更新用户昵称
///
/// 以uuid为条件更新新昵称到user_infos表的昵称字段
///
/// params:
/// - uuid: 用户uuid
/// - new_nickname: 新昵称
///
/// return: ()
pub async fn reset_nickname(uuid: &str, new_nickname: &str) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update user_infos set nickname = $1, updated_at = now() where uuid = $2;")
        .bind(new_nickname)
        .bind(uuid)
        .execute(pool)
        .await
        .context("修改用户名失败: 用户名已经存在")?;

    Ok(())
}

/// 更新用户性别
///
/// 以uuid为条件更新新性别到user_infos表的性别字段
///
/// params:
/// - uuid: 用户uuid
/// - gender: 性别
///
/// return: ()
pub async fn reset_gender(uuid: &str, gender: bool) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update user_infos set gender = $1, updated_at = now() where uuid = $2;")
        .bind(gender)
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}

/// 更新用户手机
///
/// 以uuid为条件更新手机号码到user_infos表的手机号码字段
///
/// params:
/// - uuid: 用户uuid
/// - phone: 手机号码
///
/// return: ()
pub async fn reset_phone(uuid: &str, phone: &str) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update user_infos set phone = $1, updated_at = now() where uuid = $2;")
        .bind(phone)
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}

/// 更新用户邮箱
///
/// 以uuid为条件更新邮箱到user_infos表的邮箱字段
///
/// params:
/// - uuid: 用户uuid
/// - email: 邮箱
///
/// return: ()
pub async fn reset_email(uuid: &str, email: &str) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update user_infos set email = $1, updated_at = now() where uuid = $2;")
        .bind(email)
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}

/// 为指定用户授予权限
///
/// 以uuid为条件更新用户所在的组以此来更新权限的效果。
///
/// params:
/// - uuid: 用户uuid
/// - group_id: 用户组
///
/// return: ()
pub async fn grant_permission(uuid: &str, group_id: &str) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update user_infos set email = $1, updated_at = now() where uuid = $2;")
        .bind(group_id)
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}
