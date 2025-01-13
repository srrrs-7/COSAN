use axum::{http, Json};
use chrono::{DateTime, Local, TimeZone, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::router::response::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub uid: Option<i64>,
    pub exp: Option<i64>,
    pub iat: Option<DateTime<Utc>>,
    pub scopes: Option<Vec<String>>,
    pub role: Option<String>,
}

pub fn validate_token(
    token_string: &str,
    secret_key: &str,
) -> Result<Token, (http::StatusCode, Json<ErrorResponse>)> {
    let claims = get_claims_from_token::<Token>(token_string, secret_key);
    match claims {
        Ok(claims) => {
            if let Some(exp) = claims.exp {
                let expired_datetime = Utc.timestamp_opt(exp, 0).unwrap();
                if Local::now() > expired_datetime {
                    return Err((
                        http::StatusCode::UNAUTHORIZED,
                        Json(ErrorResponse {
                            error: format!("Token is invalid"),
                            message: format!("Token is expired"),
                        }),
                    ));
                }
            }
            return Ok(claims);
        }
        Err(err) => {
            return Err((
                http::StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: format!("Token is invalid"),
                    message: format!("{}", err),
                }),
            ));
        }
    }
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
