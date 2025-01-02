use chrono::{Duration, Local, NaiveDateTime};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub uid: i64,
    pub issued: NaiveDateTime,
    pub expired: Duration,
    pub scopes: Vec<String>,
    pub role: i8,
}

pub fn validate_token(
    token_string: &str,
    secret_key: &str,
) -> Result<Token, Box<dyn std::error::Error>> {
    let claims = get_claims_from_token::<Token>(token_string, secret_key)?;

    if expired_token(claims.expired) {
        return Err("access token expired".into());
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
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

fn expired_token(expired: Duration) -> bool {
    Local::now() > Local::now() + expired
}
