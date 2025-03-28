extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};

pub fn generate_hash(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed_password = hash(password, DEFAULT_COST)?;
    Ok(hashed_password)
}

pub fn is_match(password: &str, hashed_password: &str) -> bool {
    match verify(password, hashed_password) {
        Ok(result) => result,
        Err(_) => false,
    }
}