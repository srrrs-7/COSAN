use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub uid: i64,
    pub issued: DateTime<Utc>,
    pub expired: i64,
    pub scopes: Vec<String>,
    pub role: String,
}

pub fn validate_token(
    token_string: &str,
    secret_key: &str,
) -> Result<Token, Box<dyn std::error::Error>> {
    let claims = get_claims_from_token::<Token>(token_string, secret_key)?;

    if claims.expired < Utc::now().timestamp() {
        return Err("Token has expired".into());
    }

    Ok(claims)
}

fn get_claims_from_token<T>(
    token_string: &str,
    secret_key: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let token_data = decode::<T>(
        token_string,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
