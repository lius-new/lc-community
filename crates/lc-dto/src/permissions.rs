use serde::{Deserialize, Serialize};

/// 将资源信息添加到资源表的请求参数。
#[derive(Deserialize, Serialize, Debug)]
pub struct PushResourcesRequestParam {
    pub resource_name: String,
    pub resource_description: String,
    pub resource: String,
    pub resource_method: String,
    pub resource_table: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModifyResourcesRequestParam {
    pub resource_id: i32,
    pub resource_name: String,
    pub resource_description: String,
    pub resource: String,
    pub can_use: bool,
    pub resource_method: String,
    pub resource_table: String,
}

/// 禁用某个资源的请求参数。
#[derive(Deserialize, Serialize, Debug)]
pub struct ToggleResourcesRequestParam {
    pub resource_id: i32,
    pub resource_table: String,
}

/// 将资源与权限关联的请求参数(授权)
#[derive(Deserialize, Serialize, Debug)]
pub struct GrantResourcesPermissionRequestParam {
    pub resource_id: i32,
    pub resource_permission_relation_table: String,
    pub permissions_ids: Vec<i32>,
}

/// 移除资源对应的权限的请求参数
#[derive(Deserialize, Serialize, Debug)]
pub struct RemoveResourcesPermissionRequestParam {
    pub resource_id: i32,
    pub resource_permission_relation_table: String,
    pub permissions_ids: Vec<i32>,
}

/// 显示资源对应的权限的请求参数
#[derive(Deserialize, Serialize, Debug)]
pub struct ShowResourcesPermissionRequestParam {
    pub resource_name: String,
}

/// 显示权限包哈资源的请求参数
#[derive(Deserialize, Serialize, Debug)]
pub struct ShowPermissionResourcesRequestParam {
    pub permission_id: i32,
}
