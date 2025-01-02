use chrono::{Duration, Local, NaiveDateTime};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub uid: i64,
    pub issued: NaiveDateTime,
    pub expired: Duration,
    pub scopes: Vec<String>,
    pub role: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub uid: i64,
    pub issued: NaiveDateTime,
    pub expired: Duration,
    pub scopes: Vec<String>,
    pub role: i8,
}

pub fn validate_access_token(
    token_string: &str,
) -> Result<AccessToken, Box<dyn std::error::Error>> {
    let claims = get_claims_from_token::<AccessToken>(token_string)?;

    if expired_token(claims.expired) {
        return Err("access token expired".into());
    }

    Ok(claims)
}

pub fn validate_refresh_token(
    token_string: &str,
) -> Result<RefreshToken, Box<dyn std::error::Error>> {
    let claims = get_claims_from_token::<RefreshToken>(token_string)?;

    if expired_token(claims.expired) {
        return Err("refresh token expired".into());
    }

    Ok(claims)
}

fn get_claims_from_token<T>(token_string: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let token_data = decode::<T>(
        token_string,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

fn expired_token(expired: Duration) -> bool {
    Local::now() > Local::now() + expired
}
