use anyhow::{Context, Result};
use lc_utils::database;

pub mod permissions;
pub mod users;

/// 查询指定的用户(uuid)是否拥有resource+resource_method的权限。
pub async fn auth(uuid: &str, resource: &str, resource_method: &str) -> Result<bool> {
    let sql  = "
                select ugs_us_ugrs_urgrs.user_id user_id
                from user_roles urs
                         left outer join user_role_permission_relations urprs on urs.id = urprs.user_role_id
                         left outer join (select ups.id,
                                                 ups.name,
                                                 ups.description,
                                                 ups.parent_permission_id,
                                                 purrs_urs_parrs_ars.id   as resource_id,
                                                 purrs_urs_parrs_ars.resource,
                                                 purrs_urs_parrs_ars.method,
                                                 purrs_urs_parrs_ars.Type as resource_type
                                          from user_permissions ups
                                               left outer join(select purrs.user_permission_id,
                                                                      urs.id,
                                                                      urs.resource,
                                                                      urs.method,
                                                                      'user' as Type
                                                               from permission_user_resource_relations purrs
                                                                        join user_resources urs on purrs.resource_id = urs.id

                                                               union all
                                                               select parrs.user_permission_id,
                                                                      ars.id,
                                                                      ars.resource,
                                                                      ars.method,
                                                                      'article' as Type
                                                               from permission_article_resource_relations parrs
                                                                        join article_resources ars on parrs.resource_id = ars.id

                                                               union all

                                                               select pprrs.user_permission_id,
                                                                      prs.id,
                                                                      prs.resource,
                                                                      prs.method,
                                                                      'permission' as Type
                                                               from permission_permission_resource_relations pprrs
                                                                   join permission_resources prs on pprrs.resource_id = prs.id
                                                               ) purrs_urs_parrs_ars
                                                              on ups.id = purrs_urs_parrs_ars.user_permission_id) ups_purrs_urs_parrs_ars
                                                     on urprs.user_permission_id = ups_purrs_urs_parrs_ars.id
                         left outer join (select ugs.id,
                                                 ugs.name,
                                                 ugs.description,
                                                 ugs.parent_group_id,
                                                 ugs_us_ugrs.user_id,
                                                 ugs_us_ugrs.uuid,
                                                 ugs_us_ugrs.user_group_id,
                                                 urgrs.user_role_id
                                          from user_groups ugs
                                                   left outer join (select us.id user_id, ugrs.user_group_id,us.uuid
                                                                    from users us
                                                                             inner join user_group_relations ugrs on us.uuid = ugrs.uuid) ugs_us_ugrs
                                                                   on ugs.id = ugs_us_ugrs.user_group_id
                                                   left outer join user_role_group_relations urgrs on ugs.id = urgrs.user_group_id) ugs_us_ugrs_urgrs
                                         on ugs_us_ugrs_urgrs.user_role_id = urs.id
                where ugs_us_ugrs_urgrs.uuid = $1 and ups_purrs_urs_parrs_ars.resource = $2 and ups_purrs_urs_parrs_ars.method = $3;
            ";

    let pool = database::get_connection().await?;

    let row = sqlx::query(sql)
        .bind(uuid)
        .bind(resource)
        .bind(resource_method)
        .execute(pool)
        .await
        .context("权限查询失败")?;

    Ok(row.rows_affected() > 0)
}

/// 判断该资源是否存在白名单中。
/// 白名单中的资源不需要经过权限校验。
pub async fn is_white_resource(resource: &str, resource_method: &str) -> Result<bool> {
    let pool = database::get_connection().await?;

    let result = sqlx::query(
        "select id from white_resources where resource = $1 and method = $2 and can_use = true;",
    )
    .bind(resource)
    .bind(resource_method)
    .execute(pool)
    .await;

    match result {
        Ok(row) => Ok(row.rows_affected() > 0),
        Err(_) => Ok(false),
    }
}
