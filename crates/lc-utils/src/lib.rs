use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub mod config;
pub mod database;
pub mod errors;
pub mod extract;
pub mod response;

pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
pub fn hash_password(password: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password, &salt) {
        Ok(p) => Ok(p.to_string()),
        Err(e) => Err(anyhow!("failed to hash password: {:?}", e)),
    }
}
pub fn verify_password(password: &[u8], password_hash: &str) -> bool {
    let argon2 = Argon2::default();
    if let Ok(ph) = PasswordHash::new(password_hash) {
        argon2.verify_password(password, &ph).is_ok()
    } else {
        false
    }
}
