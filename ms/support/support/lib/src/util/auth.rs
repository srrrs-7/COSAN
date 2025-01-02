use chrono::{Local, NaiveDateTime, TimeZone, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub uid: Option<i64>,
    pub issued: Option<NaiveDateTime>,
    pub expired: Option<i64>,
    pub scopes: Option<Vec<String>>,
    pub role: Option<String>,
}

pub fn validate_token(
    token_string: &str,
    secret_key: &str,
) -> Result<Token, Box<dyn std::error::Error>> {
    let claims = get_claims_from_token::<Token>(token_string, secret_key)?;

    if let Some(exp) = claims.expired {
        let expired_datetime = Utc.timestamp_opt(exp, 0).unwrap();
        if Local::now() > expired_datetime {
            return Err("access token expired".into());
        }
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
