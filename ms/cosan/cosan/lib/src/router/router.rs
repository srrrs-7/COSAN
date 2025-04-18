use crate::domain::interface;
use crate::{
    domain::service::CosanService, router::middleware, router::request, router::response, util,
};
use axum::{
    extract::{FromRequestParts, Path, State},
    http::{self, request::Parts},
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

#[derive(Clone)]
pub struct AppState<U, W, UW>
where
    U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
    W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
    UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
{
    service: Arc<CosanService<U, W, UW>>,
    secret_key: Arc<String>,
}

// Add this struct to extract the token
pub struct Token(pub Arc<util::auth::Token>);

impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, Json<response::ErrorResponse>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Extension(token) =
            Extension::<Arc<util::auth::Token>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| {
                    (
                        http::StatusCode::UNAUTHORIZED,
                        Json(response::ErrorResponse {
                            error: "Unauthorized".to_string(),
                            message: "Missing or invalid token".to_string(),
                        }),
                    )
                })?;

        Ok(Token(token))
    }
}

pub struct AppRouter {
    pub router: Router,
}

impl AppRouter {
    pub fn new<U, W, UW>(service: Arc<CosanService<U, W, UW>>, secret_key: Arc<String>) -> Self
    where
        U: interface::UserRepositoryTrait,
        W: interface::WordRepositoryTrait,
        UW: interface::UserWordRepositoryTrait,
    {
        let app_state = AppState {
            service,
            secret_key,
        };

        Self::init_router(app_state)
    }

    pub async fn serve(self) -> Result<(), anyhow::Error> {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        info!("Listening on {}", addr);

        axum::serve(
            tokio::net::TcpListener::bind(&addr).await.unwrap(),
            self.router,
        )
        .await
        .unwrap();

        Ok(())
    }

    fn init_router<U, W, UW>(state: AppState<U, W, UW>) -> AppRouter
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        let router = Router::new()
            .nest(
                "/cosan/v1",
                Router::new()
                    .route("/health", get(Self::health_check))
                    .nest(
                        "/user",
                        Router::new()
                            .route("/{user_id}", get(Self::get_user))
                            .route("/", put(Self::update_user))
                            .route("/{user_id}", delete(Self::delete_user))
                            .route_layer(axum::middleware::from_fn_with_state(
                                state.secret_key.clone(),
                                middleware::verify_token_middleware,
                            ))
                            .route("/", post(Self::create_user))
                            .route(
                                "/login/{login_id}/password/{password}",
                                get(Self::get_user_by_login_id_and_password),
                            ),
                    )
                    .nest(
                        "/word",
                        Router::new()
                            .route("/", post(Self::create_word))
                            .route("/{word_id}", get(Self::get_word))
                            .route("/", put(Self::update_word))
                            .route("/{word_id}", delete(Self::delete_word))
                            .route_layer(axum::middleware::from_fn_with_state(
                                state.secret_key.clone(),
                                middleware::verify_token_middleware,
                            )),
                    )
                    .nest(
                        "/user/word/relation",
                        Router::new()
                            .route(
                                "/user/{user_id}/word/{user_word_id}",
                                get(Self::get_user_word_by_user_id_and_word_id),
                            )
                            .route("/user/{user_id}", get(Self::get_user_word_by_user_id))
                            .route("/word/{word_id}", get(Self::get_user_word_by_word_id))
                            .route("/", post(Self::create_user_word))
                            .route("/{user_word_id}", delete(Self::delete_user_word))
                            .route_layer(axum::middleware::from_fn_with_state(
                                state.secret_key.clone(),
                                middleware::verify_token_middleware,
                            )),
                    )
                    .layer(axum::middleware::from_fn(
                        middleware::request_log_middleware,
                    )),
            )
            .with_state(state);

        AppRouter { router }
    }

    // Helper function to handle Result with custom error responses.
    async fn handle_result<T, E: ToString>(
        result: Result<T, E>,
        success_status: http::StatusCode,
        not_found_message: &str,
    ) -> Result<(http::StatusCode, Json<T>), (http::StatusCode, Json<response::ErrorResponse>)>
    {
        match result {
            Ok(value) => Ok((success_status, Json(value))),
            Err(err) => {
                let error_message = err.to_string();
                if error_message.contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(response::ErrorResponse {
                            error: error_message,
                            message: not_found_message.to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(response::ErrorResponse {
                            error: error_message,
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn health_check() -> Result<(http::StatusCode, Json<response::HealthCheckResponse>), ()> {
        info!("Health check");

        Ok((
            http::StatusCode::OK,
            Json(response::HealthCheckResponse { status: "ok" }),
        ))
    }

    async fn get_user<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Path(user_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Get user");
        info!(token = ?token);

        let user_id = i64::try_from(user_id).map_err(|_| {
            (
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Invalid user ID".to_string(),
                    message: "User ID must be a valid integer".to_string(),
                }),
            )
        })?;

        Self::handle_result(
            state.service.get_user(user_id).await,
            http::StatusCode::OK,
            "User not found",
        )
        .await
    }

    async fn create_user<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Json(body): Json<request::CreateUserRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::CreateUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Create user");

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: valid.err().unwrap().to_string(),
                }),
            ));
        }

        Self::handle_result(
            state.service.create_user(body).await,
            http::StatusCode::CREATED,
            "User already exists",
        )
        .await
    }

    async fn update_user<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Json(body): Json<request::UpdateUserRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::UpdateUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Update user");
        info!(token = ?token);

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: valid.err().unwrap().to_string(),
                }),
            ));
        }

        Self::handle_result(
            state.service.update_user(body).await,
            http::StatusCode::OK,
            "User not found",
        )
        .await
    }

    async fn delete_user<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Path(user_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::DeleteUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Delete user");
        info!(token = ?token);

        let user_id = i64::try_from(user_id).map_err(|_| {
            (
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Invalid user ID".to_string(),
                    message: "User ID must be a valid integer".to_string(),
                }),
            )
        })?;

        Self::handle_result(
            state.service.delete_user(user_id).await,
            http::StatusCode::OK,
            "User not found",
        )
        .await
    }

    async fn get_user_by_login_id_and_password<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Path(request): Path<request::GetUserRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Get user by login_id and password");

        let request = request::GetUserRequest::new(request.login_id, request.password)
            .validate()
            .await
            .map_err(|_| {
                (
                    http::StatusCode::BAD_REQUEST,
                    Json(response::ErrorResponse {
                        error: "Bad Request".to_string(),
                        message: "Invalid login ID or password".to_string(),
                    }),
                )
            })?;

        Self::handle_result(
            state
                .service
                .get_user_by_login_id_and_password(request.login_id, request.password)
                .await,
            http::StatusCode::OK,
            "User not found",
        )
        .await
    }

    async fn get_word<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Path(word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::GetWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Get word");
        info!(token = ?token);

        let word_id = i64::try_from(word_id).map_err(|_| {
            (
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Invalid user ID".to_string(),
                    message: "Word ID must be a valid integer".to_string(),
                }),
            )
        })?;

        Self::handle_result(
            state
                .service
                .get_word(i64::try_from(word_id).unwrap())
                .await,
            http::StatusCode::OK,
            "Word not found",
        )
        .await
    }

    async fn create_word<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Json(body): Json<request::CreateWordRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::CreateWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Create word");

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: valid.err().unwrap().to_string(),
                }),
            ));
        }

        Self::handle_result(
            state.service.create_word(body).await,
            http::StatusCode::CREATED,
            "Word already exists",
        )
        .await
    }

    async fn update_word<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Json(body): Json<request::UpdateWordRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::UpdateWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Update supporter");
        info!(token = ?token);

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: valid.err().unwrap().to_string(),
                }),
            ));
        }

        Self::handle_result(
            state.service.update_word(body).await,
            http::StatusCode::OK,
            "Word not found",
        )
        .await
    }

    async fn delete_word<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Path(word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::DeleteWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Delete word");
        info!(token = ?token);

        let word_id = i64::try_from(word_id).map_err(|_| {
            (
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Invalid user ID".to_string(),
                    message: "Word ID must be a valid integer".to_string(),
                }),
            )
        })?;

        Self::handle_result(
            state.service.delete_word(word_id).await,
            http::StatusCode::OK,
            "Word not found",
        )
        .await
    }

    async fn get_user_word_by_user_id_and_word_id<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Path(request): Path<request::GetUserWordRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Get user word");
        info!(token = ?token);

        let valid = request.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: valid.err().unwrap().to_string(),
                }),
            ));
        }

        Self::handle_result(
            state
                .service
                .get_user_word_by_user_id_and_word_id(request)
                .await,
            http::StatusCode::OK,
            "User word not found",
        )
        .await
    }

    async fn get_user_word_by_user_id<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Path(user_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<Vec<response::GetUserWordResponse>>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Get user word");
        info!(token = ?token);

        Self::handle_result(
            state
                .service
                .get_user_word_by_user_id(request::GetUserWordRequest {
                    user_id,
                    word_id: 0,
                })
                .await,
            http::StatusCode::OK,
            "User word not found",
        )
        .await
    }

    async fn get_user_word_by_word_id<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Path(word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<Vec<response::GetUserWordResponse>>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Get user word");
        info!(token = ?token);

        Self::handle_result(
            state
                .service
                .get_user_word_by_word_id(request::GetUserWordRequest {
                    user_id: 0,
                    word_id: u64::try_from(word_id).unwrap(),
                })
                .await,
            http::StatusCode::OK,
            "User word not found",
        )
        .await
    }

    async fn create_user_word<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Json(body): Json<request::CreateUserWordRequest>,
    ) -> Result<
        (
            http::StatusCode,
            Json<response::CreateUserWordRelationResponse>,
        ),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Create user word");
        info!(token = ?token);

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: valid.err().unwrap().to_string(),
                }),
            ));
        }

        Self::handle_result(
            state.service.create_user_word(body).await,
            http::StatusCode::CREATED,
            "User word already exists",
        )
        .await
    }

    async fn delete_user_word<U, W, UW>(
        State(state): State<AppState<U, W, UW>>,
        Token(token): Token,
        Path(user_word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::DeleteUserWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    {
        info!("Delete protagonist supporter");
        info!(token = ?token);

        Self::handle_result(
            state
                .service
                .delete_user_word(i64::try_from(user_word_id).unwrap())
                .await,
            http::StatusCode::OK,
            "Protagonist supporter not found",
        )
        .await
    }
}
