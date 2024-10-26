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

#[derive(Deserialize, Serialize, Debug)]
pub struct ResetPasswordRequestParam {
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResetNicknameRequestParam {
    pub nickname: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ResetEmailRequestParam {
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResetPhoneRequestParam {
    pub phone: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResetGenderRequestParam {
    pub gender: bool,
}
