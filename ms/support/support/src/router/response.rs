use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: &'static str,
}

impl IntoResponse for HealthCheckResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct ProtagonistSupportResponse {
    pub protagonist_id: u64,
    pub protagonist_name: String,
    pub supporter_id: u64,
    pub supporter_name: String,
}

impl IntoResponse for ProtagonistSupportResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status_code = match self.error.as_str() {
            "NotFound" => StatusCode::NOT_FOUND,
            "Unauthorized" => StatusCode::UNAUTHORIZED,
            "Forbidden" => StatusCode::FORBIDDEN,
            "BadRequest" => StatusCode::BAD_REQUEST,
            "Conflict" => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(self)).into_response()
    }
}
