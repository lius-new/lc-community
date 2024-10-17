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
