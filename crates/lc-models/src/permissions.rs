use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 查询所有的资源以及资源对应的权限 对应的结构体
#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct ResourcePermission {
    pub permission_id: i32,
    pub resource_id: i32,
    pub permission_name: String,
    pub permission_description: String,
    pub resource_permission_relation_type: String,
    pub resource_name: String,
    pub resource_type: String,
    pub resource_description: String,
}

/// 查询权限信息 对应的结构体
#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct Permissions {
    pub permission_name: String,
    pub permission_description: String,
    pub parent_permission_id: Option<i32>,
}
