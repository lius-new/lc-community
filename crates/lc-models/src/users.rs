use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 登陆账号接口中查询数据库获取到的用户信息数据对应结构体.
#[derive(FromRow, Debug)]
pub struct UserInfoWithLogin {
    pub password: String,
    pub uuid: String,
}

/// 登陆账号接口中查询数据库获取到的用户信息数据对应结构体.
#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct UserInfoWithView {
    pub nickname: String,
    pub email: Option<String>,
    pub gender: Option<bool>,
    pub phone: Option<String>,
}
