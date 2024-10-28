use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use data_encoding::HEXUPPER;
use hmac::{Hmac, Mac};
use jwt::{Header, RegisteredClaims, SignWithKey, Token, VerifyWithKey};
use lazy_static::lazy_static;

use ring::digest::{Context, SHA256};

use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    time::{self},
};

pub mod config;
pub mod database;
pub mod errors;
pub mod extract;
pub mod response;

lazy_static! {
    /// 构建token时的密钥对
    pub static ref SECRETY_KEY: Hmac<Sha256> = Hmac::new_from_slice(b"some-simple-secret-ls").unwrap();
    /// 对uuid加密的 rsp 密钥. (私钥(用于解密)、公钥(用于解密))
    pub static ref RSA_KEY_WITH_UUID: (RsaPrivateKey,RsaPublicKey) = generate_rsa_keys().unwrap();
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
/// - v: 需要被签名的value (通常是uuid)
pub fn sign_with_value(v: &str) -> Result<String> {
    let header = Header {
        algorithm: jwt::AlgorithmType::Hs256,
        ..Default::default()
    };

    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let encrypt_uuid = encrypt_str(v, &RSA_KEY_WITH_UUID.1)?;

    let claims = RegisteredClaims {
        issuer: Some("your-issuer".to_string()),
        subject: Some("user123".to_string()),
        audience: Some("your-audience".to_string()),
        expiration: Some((now + 3600 * 1000 * 12) as u64), // 1小时后过期
        not_before: Some(now as u64),
        issued_at: Some(now as u64),
        json_web_token_id: Some(encrypt_uuid),
    };

    let token = Token::new(header, claims)
        .sign_with_key(&*SECRETY_KEY)
        .map_err(|e| anyhow!("{}", e))?;

    Ok(token.into())
}

/// 判断是否超过时间
pub fn out_expiration(expiration: Option<u64>) -> bool {
    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    expiration.map_or_else(|| false, |exp| (exp as i64 - now) < 0)
}

/// 校验签名(token)
/// params:
/// - token_str: 签名
pub fn verify_sign_with_token(token_str: &str) -> Result<(String, bool)> {
    let token: Token<Header, RegisteredClaims, _> = token_str.verify_with_key(&*SECRETY_KEY)?;
    let claims = token.claims();

    if out_expiration(claims.expiration) {
        return Ok(("".to_string(), false));
    }

    match &claims.json_web_token_id {
        Some(encrypted_uuid) => Ok((decrypt_str(encrypted_uuid, &RSA_KEY_WITH_UUID.0)?, true)),
        _ => Ok(("".to_string(), false)),
    }
}

/// 生成rsa密钥对
pub fn generate_rsa_keys() -> Result<(RsaPrivateKey, RsaPublicKey)> {
    let mut rng = OsRng;
    let bits = 2024;
    let priv_key = RsaPrivateKey::new(&mut rng, bits)?;
    let pub_key = RsaPublicKey::from(&priv_key);

    Ok((priv_key, pub_key))
}

/// 对uuid进行加密
pub fn encrypt_str(str_: &str, public_key: &RsaPublicKey) -> Result<String> {
    let encrypted = public_key.encrypt(&mut OsRng, Pkcs1v15Encrypt, str_.as_bytes())?;

    Ok(base64::encode(&encrypted))
}

/// 对uuid进行解密
pub fn decrypt_str(encrypted: &str, private_key: &RsaPrivateKey) -> Result<String> {
    let decrypted = private_key.decrypt(Pkcs1v15Encrypt, &base64::decode(encrypted)?)?;

    Ok(String::from_utf8(decrypted)?)
}

/// 对uuid进行解密
pub fn hash(content: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();

    content.hash(&mut hasher);

    hasher.finish()
}

/// 计算文件hash
pub fn sha256_digest(content: &[u8]) -> Result<String> {
    let mut context = Context::new(&SHA256);

    context.update(content);

    Ok(HEXUPPER.encode(context.finish().as_ref()))
}

///// 计算文件hash
//pub fn newpwd(password: &str) -> Result<String> {
//    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
//
//    let mut pbkdf2_hash = [0u8; digest::SHA512_OUTPUT_LEN];
//
//    let rng = rand::SystemRandom::new();
//    let mut salt = [0u8; CREDENTIAL_LEN];
//    rng.fill(&mut salt).unwrap();
//
//    pbkdf2::derive(
//        PBKDF2_HMAC_SHA512,
//        NonZeroU32::new(100_000).unwrap(),
//        &salt,
//        password.as_bytes(),
//        &mut pbkdf2_hash,
//    );
//
//    Ok(HEXUPPER.encode(&pbkdf2_hash))
//}
//
//pub fn newpwd_verify(password: &str, hash: &str) -> bool {
//    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
//    let rng = rand::SystemRandom::new();
//    let mut salt = [0u8; CREDENTIAL_LEN];
//    rng.fill(&mut salt).unwrap();
//
//    pbkdf2::verify(
//        pbkdf2::PBKDF2_HMAC_SHA256,
//        NonZeroU32::new(100_000).unwrap(),
//        &salt,
//        password.as_bytes(),
//        hash.as_bytes(),
//    )
//    .is_ok()
//}
