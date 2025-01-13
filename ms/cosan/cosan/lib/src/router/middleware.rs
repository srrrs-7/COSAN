use super::response::ErrorResponse;
use crate::util;
use axum::{extract::State, http, middleware::Next, response::Response, Json};
use std::sync::Arc;
use tracing::{error, info};

pub async fn verify_token_middleware<B>(
    State(secret_key): State<Arc<String>>,
    mut req: http::Request<B>,
    next: Next<B>,
) -> Result<Response, (http::StatusCode, Json<ErrorResponse>)> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        error!("verify_token_middleware: Authorization header not found");
        return Err((
            http::StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: format!("Token is invalid"),
                message: format!("Authorization header not found"),
            }),
        ));
    };

    let bearer = auth_header.split_whitespace().nth(0).unwrap_or_default();
    if bearer != "Bearer" {
        error!("verify_token_middleware: Authorization header is not Bearer");
        return Err((
            http::StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: format!("Token is invalid"),
                message: format!("Authorization header is not Bearer"),
            }),
        ));
    }

    let token = auth_header.split_whitespace().nth(1).unwrap_or_default();
    if token.is_empty() {
        error!("verify_token_middleware: Token is empty");
        return Err((
            http::StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: format!("Token is invalid"),
                message: format!("Token is empty"),
            }),
        ));
    }

    let token = util::auth::validate_token(token, &secret_key);
    match token {
        Ok(token) => {
            info!("verify_token_middleware: Token is valid");
            // token info add to request context
            req.extensions_mut().insert(Arc::new(token));

            Ok(next.run(req).await)
        }
        Err(err) => {
            error!("verify_token_middleware: Token is invalid");
            return Err(err);
        }
    }
}

pub async fn request_log_middleware<B>(
    req: http::Request<B>,
    next: Next<B>,
) -> Result<Response, http::StatusCode> {
    info!("Request: {} {}", req.method(), req.uri());
    let res = next.run(req).await;
    info!("Response: {}", res.status());

    Ok(res)
}
