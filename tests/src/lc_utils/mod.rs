pub mod config;

/// 测试uuid生成
#[test]
fn test_uuid() {
    let uuid_str = lc_utils::uuid();
    println!("{}", uuid_str)
}

/// 测试生成密码
#[test]
fn test_hash_password() {
    let _ = lc_utils::hash_password(b"abc");
}

/// 测试校验密码
#[test]
fn test_verify_password() {
    let password = "1".as_bytes();
    let password_hash =  "$argon2id$v=19$m=19456,t=2,p=1$MPZeayHODZCNHJlrhG+Xjw$03F3y5b0elI7bz27b+rGHI2BWNEm/WU0KEmv0GbfqT0";

    let _ = lc_utils::verify_password(password, password_hash);
}

/// 测试生成签名(token)
#[test]
fn test_sign_with_value() {
    let _ = lc_utils::sign_with_value("abc");
}

/// 测试校验签名
#[test]
fn test_verify_sign_with_token() {
    let token_str = lc_utils::sign_with_value("abc").unwrap();

    let (token_str, ok) = lc_utils::verify_sign_with_token(&token_str).unwrap();

    println!("{:?} {:?}", token_str, ok)
}

/// 测试生成rsa key
#[test]
fn test_generate_rsa_keys() {
    let (pri, pubk) = lc_utils::generate_rsa_keys().unwrap();

    println!("{:?} {:?}", pri, pubk)
}

/// 测试加密uuid
#[test]
fn test_encrypt_uuid() {
    let (_, pubk) = lc_utils::generate_rsa_keys().unwrap();

    let res = lc_utils::encrypt_str("abc", &pubk);
    println!("{:?} ", res,)
}

/// 测试解密uuid
#[test]
fn test_decrypt_uuid() {
    let (private_key, pubk) = lc_utils::generate_rsa_keys().unwrap();

    let encrypted = lc_utils::encrypt_str("abc", &pubk).unwrap();
    println!(" encrypted: {:?} ", encrypted);

    let value = lc_utils::decrypt_str(&encrypted, &private_key);
    println!("value: {:?} ", value,);
}

/// 测试对字符串进行hash计算
#[test]
fn test_hash_str() {
    let hash_ = lc_utils::hash(b"abc");

    println!("{:?}", hash_);
}

/// 测试对文件进行hash计算
#[test]
fn test_sha256_digest() {
    let hash_ = lc_utils::sha256_digest(b"abc").unwrap();

    println!("{}", hash_)
}
