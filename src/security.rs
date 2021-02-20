use argon2::{self, Config};

pub fn gen_password_hash(pwd: &str) -> String {
    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(pwd.as_bytes(), salt, &config).unwrap();

    return hash;
}

pub fn verify_password(pwd: &str, hash: &str) -> bool {
    return argon2::verify_encoded(&hash, pwd.as_bytes())
        .unwrap();
}
