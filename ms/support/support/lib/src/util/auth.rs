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

#[test]
fn test_validate_token_valid() {
    let secret_key = "secret_key";
    let token = Token {
        uid: 1,
        issued: Local::now().naive_local(),
        expired: Duration::hours(1),
        scopes: vec!["read".to_string()],
        role: 1,
    };
    let token_string = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &token,
        &jsonwebtoken::EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap();

    let result = validate_token(&token_string, secret_key);
    assert!(result.is_ok());
    let validated_token = result.unwrap();
    assert_eq!(validated_token.uid, 1);
    assert_eq!(validated_token.scopes, vec!["read".to_string()]);
    assert_eq!(validated_token.role, 1);
}

#[test]
fn test_validate_token_expired() {
    let secret_key = "secret_key";
    let token = Token {
        uid: 1,
        issued: Local::now().naive_local(),
        expired: Duration::seconds(-1), // expired token
        scopes: vec!["read".to_string()],
        role: 1,
    };
    let token_string = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &token,
        &jsonwebtoken::EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap();

    let result = validate_token(&token_string, secret_key);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "access token expired");
}

#[test]
fn test_validate_token_invalid_secret() {
    let secret_key = "secret_key";
    let token = Token {
        uid: 1,
        issued: Local::now().naive_local(),
        expired: Duration::hours(1),
        scopes: vec!["read".to_string()],
        role: 1,
    };
    let token_string = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &token,
        &jsonwebtoken::EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap();

    let result = validate_token(&token_string, "wrong_secret_key");
    assert!(result.is_err());
}

#[test]
fn test_expired_token() {
    let expired = Duration::seconds(-1);
    assert!(expired_token(expired));

    let expired = Duration::seconds(1);
    assert!(!expired_token(expired));
}
