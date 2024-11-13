use axum::{
    routing::{get, post},
    Router,
};

use resources::*;

pub mod api_management {
    use super::*;
    pub fn build_router() -> Router {
        Router::new().nest(
            "/permissions",
            Router::new()
                .nest(
                    "/resources",
                    Router::new()
                        .route("/show-category", post(show_resource_category))
                        .route("/push", post(push))
                        .route("/modify", post(modify))
                        .route("/toggle-canuse", post(toggle_canuse))
                        .route("/grant-permissions", post(grant_permissions))
                        .route("/remove-permissions", post(remove_permissions))
                        .route("/show-resources", post(show_allresources_permissions))
                        .route("/show-current-resources", post(show_current_resources))
                        .route("/show-permissions", post(show_current_permissions)),
                )
                .route(
                    "/show-all-permissions",
                    get(permissions::show_all_permissions),
                ),
        )
    }
}
pub mod api {
    use super::*;
    pub fn build_router() -> Router {
        Router::new()
    }
}

mod permissions {
    use lc_models::permissions::Permissions;
    use lc_services::permissions;
    use lc_utils::response::Response;

    /// 显示指定权限把含的资源信息
    pub async fn show_all_permissions() -> Response<Vec<Permissions>> {
        let resp = Response::default();

        match permissions::show_all_permissions().await {
            Ok(value) => resp.success("获取权限资源信息成功", Some(value)),
            Err(e) => resp.fail("获取权限资源信息失败", Some(e)),
        }
    }
}

mod resources {
    use axum::response::Result;
    use lc_dto::permissions;
    use lc_models::permissions::ResourcePermission;
    use lc_services::permissions as permission_services;
    use lc_utils::{errors::AppError, extract::Json, response::Response};

    /// 显示所有的资源表, 在执行资源操作时需要指定该资源位于何处。
    pub async fn show_resource_category() -> Result<Response<Vec<String>>, AppError> {
        let resource_category = permission_services::show_all_resource_type().await?;

        Ok(Response::default().success(
            "获取资源类别成功",
            Some(
                resource_category
                    .iter()
                    .map(|v| v.0.to_string())
                    .collect::<Vec<String>>(),
            ),
        ))
    }

    /// 显示所有的资源表, 在执行资源操作时需要指定该资源位于何处。
    pub async fn show_allresources_permissions(
    ) -> Result<Response<Vec<ResourcePermission>>, AppError> {
        let resource_permission =
            permission_services::show_allresource_permissions(None, None).await?;
        Ok(Response::default().success("获取资源权限成功", Some(resource_permission)))
    }

    /// 添加资源(该资源表示接口资源，用作权限系统)
    pub async fn push(
        Json(payload): Json<permissions::PushResourcesRequestParam>,
    ) -> Result<Response<()>, AppError> {
        permission_services::push_resources(payload).await?;
        Ok(Response::default().success("添加资源成功", Some(())))
    }

    /// 添加资源(该资源表示接口资源，用作权限系统)
    pub async fn modify(
        Json(payload): Json<permissions::ModifyResourcesRequestParam>,
    ) -> Result<Response<()>, AppError> {
        permission_services::modify_resources(payload).await?;
        Ok(Response::default().success("修改资源成功", Some(())))
    }

    /// 资源切换是否可用
    pub async fn toggle_canuse(
        Json(payload): Json<permissions::ToggleResourcesRequestParam>,
    ) -> Result<Response<()>, AppError> {
        permission_services::toggle_canuse_resources(
            payload.resource_id,
            payload.resource_table.as_str(),
        )
        .await?;

        Ok(Response::default().success("资源是否可用切换成功", Some(())))
    }

    /// 授予资源权限
    pub async fn grant_permissions(
        Json(payload): Json<permissions::GrantResourcesPermissionRequestParam>,
    ) -> Result<Response<()>, AppError> {
        permission_services::grant_permissions_with_resources(
            payload.resource_id,
            payload.resource_permission_relation_table.as_str(),
            payload.permissions_ids,
        )
        .await?;
        Ok(Response::default().success("资源授权成功", Some(())))
    }

    /// 修改资源权限
    pub async fn remove_permissions(
        Json(payload): Json<permissions::RemoveResourcesPermissionRequestParam>,
    ) -> Result<Response<()>, AppError> {
        permission_services::remove_permissions_with_resources(
            payload.resource_id,
            payload.resource_permission_relation_table.as_str(),
            payload.permissions_ids,
        )
        .await?;
        Ok(Response::default().success("资源移除权限成功", Some(())))
    }

    /// 显示指定资源对应的权限信息
    pub async fn show_current_resources(
        Json(payload): Json<permissions::ShowResourcesPermissionRequestParam>,
    ) -> Result<Response<Vec<ResourcePermission>>, AppError> {
        let crruent_resources = permission_services::show_permissions_with_current_resources(
            payload.resource_name.as_str(),
        )
        .await?;
        Ok(Response::default().success("获取资源权限信息成功", Some(crruent_resources)))
    }

    /// 显示指定权限包含的资源信息
    pub async fn show_current_permissions(
        Json(payload): Json<permissions::ShowPermissionResourcesRequestParam>,
    ) -> Result<Response<Vec<ResourcePermission>>, AppError> {
        let current_permisisons =
            permission_services::show_permissions_with_current_permissions(payload.permission_id)
                .await?;
        Ok(Response::default().success("获取权限资源信息成功", Some(current_permisisons)))
    }
}
