use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use lazy_static::lazy_static;
use sha2::Sha256;

pub mod config;
pub mod database;
pub mod errors;
pub mod extract;
pub mod response;

lazy_static! {
    pub static ref SECRETY_KEY: Hmac<Sha256> =
        Hmac::new_from_slice(b"some-simple-secret-ls").unwrap();
}

/// 生成uuid
pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
/// 对密码进行hash
/// params:
///    - password: 需要被hash加密的密码.
pub fn hash_password(password: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password, &salt) {
        Ok(p) => Ok(p.to_string()),
        Err(e) => Err(anyhow!("failed to hash password: {:?}", e)),
    }
}
/// 检查密码是否正确
/// params:
///    - password: 需要被检查的原始密码
///    - password_hash: 被hash加密过的密码.
pub fn verify_password(password: &[u8], password_hash: &str) -> bool {
    let argon2 = Argon2::default();
    if let Ok(ph) = PasswordHash::new(password_hash) {
        argon2.verify_password(password, &ph).is_ok()
    } else {
        false
    }
}

/// 发布签名(token)
/// params:
/// - v: 需要被签名的value
pub fn sign_with_value(v: &str) -> Result<String> {
    let header = Header {
        algorithm: jwt::AlgorithmType::Hs256,
        ..Default::default()
    };

    let mut claims = BTreeMap::new();
    claims.insert("_key", v);

    let token = Token::new(header, claims)
        .sign_with_key(&*SECRETY_KEY)
        .map_err(|e| anyhow!("{}", e))?;

    Ok(token.into())
}

/// 校验签名(token)
/// params:
/// - token_str: 签名
pub fn verify_sign_with_token(token_str: &str) -> Result<(String, bool)> {
    let claims: BTreeMap<String, String> = token_str.verify_with_key(&*SECRETY_KEY)?;

    match claims.get("_key") {
        Some(v) => Ok((v.to_string(), true)),
        None => Ok(("".to_string(), false)),
    }
}
