pub mod articles;
pub mod permissions;
pub mod users;

use serde::{Deserialize, Serialize};

/// 分页获取数据的请求参数
#[derive(Deserialize, Serialize, Debug)]
pub struct PageRequestParams {
    pub page_num: i32,
    pub page_size: i32,
}
