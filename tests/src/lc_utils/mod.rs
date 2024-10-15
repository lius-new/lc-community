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
