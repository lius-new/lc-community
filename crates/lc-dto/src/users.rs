use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterRequestParam {
    pub nickname: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginRequestParam {
    pub nickname: String,
    pub password: String,
}
