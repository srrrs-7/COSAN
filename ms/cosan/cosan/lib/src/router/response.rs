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
pub struct GetWordResponse {
    pub word_id: u64,
    pub word: String,
}

impl IntoResponse for GetWordResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateWordResponse {
    pub word_id: u64,
    pub word: String,
}

impl IntoResponse for CreateWordResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct UpdateWordResponse {
    pub word_id: u64,
    pub word: String,
}

impl IntoResponse for UpdateWordResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct DeleteWordResponse {
    pub status: String,
}

impl IntoResponse for DeleteWordResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct GetUserWordResponse {
    pub user_word_id: u64,
    pub user_id: u64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
    pub word_id: u64,
    pub word: String,
    pub created_at: String,
}

impl IntoResponse for GetUserWordResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateUserWordResponse {
    pub user_word_id: u64,
    pub user_id: u64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
    pub word_id: u64,
    pub word: String,
    pub created_at: String,
}

impl IntoResponse for CreateUserWordResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateUserWordRelationResponse {
    pub user_id: u64,
    pub word_id: u64,
    pub created_at: String,
}

impl IntoResponse for CreateUserWordRelationResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct DeleteUserWordResponse {
    pub status: String,
}

impl IntoResponse for DeleteUserWordResponse {
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
