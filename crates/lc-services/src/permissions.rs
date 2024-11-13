use anyhow::Result;
use lc_dto::permissions;
use lc_models::permissions::{Permissions, ResourcePermission};
use lc_utils::database;

/// 显示数据库中的资源表
pub async fn show_all_resource_type() -> Result<Vec<(String,)>> {
    let pool = database::get_connection().await?;

    let resource_tables: Vec<(String,)> = sqlx::query_as(
        "select tablename from pg_tables where schemaname like 'public' and tablename != 'white_resources' and tablename like '%resources';",
    )
    .fetch_all(pool)
    .await?;

    Ok(resource_tables)
}

/// 显示数据库中的资源权限关联表
pub async fn show_all_rp_type() -> Result<Vec<(String,)>> {
    let pool = database::get_connection().await?;

    let rp_relations_tables: Vec<(String,)> = sqlx::query_as(
        "select tablename from pg_tables where schemaname like 'public' and tablename like '%permission_%_resource_relations';",
    )
    .fetch_all(pool)
    .await?;

    Ok(rp_relations_tables)
}

/// 显示所有权限信息
pub async fn show_all_permissions() -> Result<Vec<Permissions>> {
    let pool = database::get_connection().await?;

    let rp_relations_tables: Vec<Permissions> =
        sqlx::query_as("select name  permission_name, description permission_description, parent_permission_id from user_permissions;")
            .fetch_all(pool)
            .await?;

    Ok(rp_relations_tables)
}

/// 显示所有的资源以及资源对应的权限.
/// params;
/// - resource_name: 如果该参数为Some, 则查询指定资源的权限信息。
/// - permission_id: 如果该参数为Some, 则查询指定权限的对应的资源权限信息。
///
/// 两参数同时存在均失效。
pub async fn show_allresource_permissions(
    resource_name: Option<&str>,
    permission_id: Option<i32>,
) -> Result<Vec<ResourcePermission>> {
    let pool = database::get_connection().await?;

    // 先查询到资源表格
    let resource_tables: Vec<(String,)> = show_all_resource_type().await?;

    // 连接所有的资源表
    let sql_resource = resource_tables
        .iter()
        .map(|(table_name,)| {
            format!(
                "select id, name, description, resource, can_use, method, '{}' as resource_type from {}",
                table_name, table_name
            )
        })
        .collect::<Vec<String>>()
        .join(" union all ");

    // 查询所有资源权限关联表格。
    let rp_relations_tables: Vec<(String,)> = show_all_rp_type().await?;

    // 连接所有资源权限关联表格。
    let sql_rp = rp_relations_tables
        .iter()
        .map(|(table_name,)| format!("select resource_id, user_permission_id,'{}' as resource_permission_relation_type  from {}", table_name, table_name))
        .collect::<Vec<String>>()
        .join(" union all ");

    let mut sql = format!("
                            select up.id                   permission_id,
                                   up.name                 permission_name,
                                   up.description          permission_description,
                                   resource_id,
                                   ar.name                 resource_name,
                                   ar.resource_type        resource_type,
                                   ar.description          resource_description,
                                   prs.resource_permission_relation_type resource_permission_relation_type
                            from ( {} ) as prs
                                     left outer join user_permissions up on up.id = prs.user_permission_id
                                     left outer join ( {} ) ar on ar.id = resource_id "
       , sql_rp,sql_resource);

    // 如果参数存在就追加对应sql完成筛选。
    if resource_name.is_some() && permission_id.is_none() {
        sql += format!("where ar.name = '{}'", resource_name.unwrap()).as_ref();
    }
    if resource_name.is_none() && permission_id.is_some() {
        sql += format!("where up.id = '{}'", permission_id.unwrap()).as_ref();
    }

    // sql command:
    // select  up.id,resource_id,  ar.name resource_name, ar.description resource_description, up.name permission_name,up.description permission_description
    // from (select resource_id,user_permission_id from permission_user_resource_relations union all select resource_id,user_permission_id  from permission_article_resource_relations) ap
    //         left outer join user_permissions up on up.id = ap.user_permission_id
    //         left outer join (select id, name, description, resource, can_use, method
    //                          from user_resources
    //                          union all
    //                          select id, name, description, resource, can_use, method
    //                          from article_resources) ar on ar.id = resource_id;
    // -- where ar.name = 'GET+/api/users/logout';
    // where up.id = '1';
    let result: Vec<ResourcePermission> = sqlx::query_as(sql.as_ref())
        .bind(sql_resource)
        .bind(sql_rp)
        .fetch_all(pool)
        .await?;

    Ok(result)
}

/// 添加资源到资源表中。
pub async fn push_resources(payload: permissions::PushResourcesRequestParam) -> Result<()> {
    let pool = database::get_connection().await?;

    let sql = format!(
        "insert into {} (name, description, resource, method)  values ($1, $2, $3, $4);",
        payload.resource_table
    );

    sqlx::query(sql.as_str())
        .bind(payload.resource_name)
        .bind(payload.resource_description)
        .bind(payload.resource)
        .bind(payload.resource_method)
        .execute(pool)
        .await?;

    Ok(())
}

/// 修改资源信息
pub async fn modify_resources(payload: permissions::ModifyResourcesRequestParam) -> Result<()> {
    let pool = database::get_connection().await?;

    sqlx::query("update $1 set name = $2, description = $3, resource = $4, can_use = $5, method = $6, updated_at = now() where id = $7;")
        .bind(payload.resource_table)
        .bind(payload.resource_name)
        .bind(payload.resource_description)
        .bind(payload.resource)
        .bind(payload.can_use)
        .bind(payload.resource_method)
        .bind(payload.resource_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// 资源是否允许访问
pub async fn toggle_canuse_resources(reosurce_id: i32, resource_table: &str) -> Result<()> {
    let pool = database::get_connection().await?;

    let can_use: (bool,) =
        sqlx::query_as(format!("select can_use from {} where id = $1;", resource_table).as_ref())
            .bind(reosurce_id)
            .fetch_one(pool)
            .await?;

    sqlx::query(
        format!(
            "update {} set can_use = $1, updated_at = now() where id = $2;",
            resource_table
        )
        .as_ref(),
    )
    .bind(!can_use.0)
    .bind(reosurce_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// 为资源授权
pub async fn grant_permissions_with_resources(
    reosurce_id: i32,
    resource_permission_relation_table: &str,
    permission_ids: Vec<i32>,
) -> Result<()> {
    let pool = database::get_connection().await?;

    let mut tx = pool.begin().await?;

    for pid in permission_ids {
        let sql = format!(
            "insert into {}(resource_id, user_permission_id)  values ($1, $2);",
            resource_permission_relation_table
        );

        sqlx::query(sql.as_ref())
            .bind(reosurce_id)
            .bind(pid)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}

/// 移除资源权限
pub async fn remove_permissions_with_resources(
    reosurce_id: i32,
    resource_permission_relation_table: &str,
    permission_ids: Vec<i32>,
) -> Result<()> {
    let pool = database::get_connection().await?;

    let mut tx = pool.begin().await?;

    for pid in permission_ids {
        let sql = format!(
            "delete from {} where resource_id = $1 and user_permission_id = $2;",
            resource_permission_relation_table
        );

        sqlx::query(sql.as_ref())
            .bind(reosurce_id)
            .bind(pid)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}

/// 显示指定资源权限。
pub async fn show_permissions_with_current_resources(
    resource_name: &str,
) -> Result<Vec<ResourcePermission>> {
    let current_resource_permissions =
        show_allresource_permissions(Some(resource_name), None).await?;

    Ok(current_resource_permissions)
}

/// 根据权限来筛选 显示指定资源权限。
pub async fn show_permissions_with_current_permissions(
    permission_id: i32,
) -> Result<Vec<ResourcePermission>> {
    let current_resource_permissions =
        show_allresource_permissions(None, Some(permission_id)).await?;

    Ok(current_resource_permissions)
}
