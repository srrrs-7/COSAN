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
pub struct GetProtagonistResponse {
    pub id: u64,
    pub name: String,
}

impl IntoResponse for GetProtagonistResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateProtagonistResponse {
    pub status: String,
}

impl IntoResponse for CreateProtagonistResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct UpdateProtagonistResponse {
    pub status: String,
}

impl IntoResponse for UpdateProtagonistResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct DeleteProtagonistResponse {
    pub status: String,
}

impl IntoResponse for DeleteProtagonistResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct GetSupporterResponse {
    pub id: u64,
    pub name: String,
}

impl IntoResponse for GetSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateSupporterResponse {
    pub status: String,
}

impl IntoResponse for CreateSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct UpdateSupporterResponse {
    pub status: String,
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
    pub protagonist_id: u64,
    pub protagonist_name: String,
    pub supporter_id: u64,
    pub supporter_name: String,
}

impl IntoResponse for GetProtagonistSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct CreateProtagonistSupporterResponse {
    pub status: String,
}

impl IntoResponse for CreateProtagonistSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct UpdateProtagonistSupporterResponse {
    pub status: String,
}

impl IntoResponse for UpdateProtagonistSupporterResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
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
