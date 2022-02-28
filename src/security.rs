use argon2::{self, Config};
use uuid::Uuid;

pub fn gen_password_hash(pwd: &str) -> String {
    let salt_str = Uuid::new_v4().to_simple().to_string();
    let salt_bytes = salt_str.as_bytes();
    let config = Config::default();
    let pwd_bytes = pwd.as_bytes();
    
    let hash = argon2::hash_encoded(pwd_bytes, &salt_bytes, &config).unwrap();

    return hash;
}

pub fn verify_password(pwd: &str, hash: &str) -> bool {
    return argon2::verify_encoded(&hash, pwd.as_bytes())
        .unwrap();
}
