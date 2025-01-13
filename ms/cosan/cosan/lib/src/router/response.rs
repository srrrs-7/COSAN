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
pub struct GetUserResponse {
    pub user_id: u64,
    pub user_last_name: String,
    pub user_first_name: String,
    pub user_email: String,
    pub user_country: String,
}

impl IntoResponse for GetUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub user_id: u64,
    pub user_last_name: String,
    pub user_first_name: String,
    pub user_email: String,
    pub user_country: String,
}

impl IntoResponse for CreateUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct UpdateUserResponse {
    pub user_id: u64,
    pub user_last_name: String,
    pub user_first_name: String,
    pub user_email: String,
    pub user_country: String,
}

impl IntoResponse for UpdateUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct DeleteUserResponse {
    pub status: String,
}

impl IntoResponse for DeleteUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct GetSupporterResponse {
    pub supporter_id: u64,
    pub supporter_last_name: String,
    pub supporter_first_name: String,
    pub supporter_email: String,
    pub supporter_country: String,
}

impl IntoResponse for GetSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateSupporterResponse {
    pub supporter_id: u64,
    pub supporter_last_name: String,
    pub supporter_first_name: String,
    pub supporter_email: String,
    pub supporter_country: String,
}

impl IntoResponse for CreateSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct UpdateSupporterResponse {
    pub supporter_id: u64,
    pub supporter_last_name: String,
    pub supporter_first_name: String,
    pub supporter_email: String,
    pub supporter_country: String,
}

impl IntoResponse for UpdateSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct DeleteSupporterResponse {
    pub status: String,
}

impl IntoResponse for DeleteSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct GetProtagonistSupporterResponse {
    pub supporter_id: u64,
    pub last_name: String,
    pub first_name: String,
    pub country: String,
}

impl IntoResponse for GetProtagonistSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateProtagonistSupporterResponse {
    pub protagonist_supporter_id: u64,
}

impl IntoResponse for CreateProtagonistSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct DeleteProtagonistSupporterResponse {
    pub status: String,
}

impl IntoResponse for DeleteProtagonistSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize)]
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
