use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub async fn verify_password(
    password: &str,
    hashed_password: &str,
) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed_password)
}
