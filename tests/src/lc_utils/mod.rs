pub mod config;

#[test]
fn test_uuid() {
    let uuid_str = lc_utils::uuid();
    println!("{}", uuid_str)
}
#[test]
fn test_hash_password() {
    let pwd_hash_str = lc_utils::hash_password(b"abc");
    println!("{:?}", pwd_hash_str)
}

#[test]
fn test_sign_with_value() {
    let token_str = lc_utils::sign_with_value("abc");
    println!("{:?}", token_str)
}

#[test]
fn test_verify_sign_with_token() {
    let token_str = lc_utils::sign_with_value("abc").unwrap();

    let (token_str, ok) = lc_utils::verify_sign_with_token(&token_str).unwrap();

    println!("{:?} {:?}", token_str, ok)
}

#[test]
fn test_generate_rsa_keys() {
    let (pri, pubk) = lc_utils::generate_rsa_keys().unwrap();

    println!("{:?} {:?}", pri, pubk)
}

#[test]
fn test_encrypt_uuid() {
    let (_, pubk) = lc_utils::generate_rsa_keys().unwrap();

    let res = lc_utils::encrypt_uuid("abc", &pubk);
    println!("{:?} ", res,)
}
#[test]
fn test_decrypt_uuid() {
    let (private_key, pubk) = lc_utils::generate_rsa_keys().unwrap();

    let encrypted = lc_utils::encrypt_uuid("abc", &pubk).unwrap();
    println!(" encrypted: {:?} ", encrypted);

    let value = lc_utils::decrypt_uuid(&encrypted, &private_key);
    println!("value: {:?} ", value,);
}
