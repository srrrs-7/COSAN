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
pub struct AppRouter<U, W, UW>
where
    U: interface::UserRepositoryTrait + Send + Sync + 'static,
    W: interface::WordRepositoryTrait + Send + Sync + 'static,
    UW: interface::UserWordRepositoryTrait + Send + Sync + 'static,
{
    service: CosanService<U, W, UW>,
    secret_key: String,
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

impl<U, W, UW> AppRouter<U, W, UW>
where
    U: interface::UserRepositoryTrait + Send + Sync + 'static,
    W: interface::WordRepositoryTrait + Send + Sync + 'static,
    UW: interface::UserWordRepositoryTrait + Send + Sync,
{
    pub fn new(service: CosanService<U, W, UW>, secret_key: String) -> Self {
        Self {
            service,
            secret_key,
        }
    }

    pub async fn serve(&self) -> Result<(), anyhow::Error> {
        let router = Self::init_router(self).await?;
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        info!("Listening on {}", addr);

        axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), router)
            .await
            .unwrap();

        Ok(())
    }

    async fn init_router(&self) -> Result<Router, anyhow::Error> {
        let arc_secret_key = Arc::new(self.secret_key.clone());

        let router =
            Router::new()
                .nest(
                    "/cosan/v1",
                    Router::new()
                        .route("/health", get(Self::health_check))
                        .nest(
                            "/user",
                            Router::new()
                                .route(
                                    "/:user_id",
                                    get(|state, token, path| async move {
                                        Self::get_user(state, token, path).await
                                    }),
                                )
                                .route(
                                    "/",
                                    put(|state, token, body| async move {
                                        Self::update_user(state, token, body).await
                                    }),
                                )
                                .route(
                                    "/:user_id",
                                    delete(|state, token, path| async move {
                                        Self::delete_user(state, token, path).await
                                    }),
                                )
                                .route_layer(axum::middleware::from_fn_with_state(
                                    arc_secret_key.clone(),
                                    middleware::verify_token_middleware,
                                ))
                                .route(
                                    "/",
                                    post(|state, body| async move {
                                        Self::create_user(state, body).await
                                    }),
                                )
                                .route(
                                    "/login/:login_id/password/:password",
                                    get(|state, path| async move {
                                        Self::get_user_by_login_id_and_password(state, path).await
                                    }),
                                ),
                        )
                        .nest(
                            "/word",
                            Router::new()
                                .route(
                                    "/",
                                    post(|state, body| async move {
                                        Self::create_word(state, body).await
                                    }),
                                )
                                .route(
                                    "/:word_id",
                                    get(|state, token, path| async move {
                                        Self::get_word(state, token, path).await
                                    }),
                                )
                                .route(
                                    "/",
                                    put(|state, token, body| async move {
                                        Self::update_word(state, token, body).await
                                    }),
                                )
                                .route(
                                    "/:word_id",
                                    delete(|state, token, path| async move {
                                        Self::delete_word(state, token, path).await
                                    }),
                                )
                                .route_layer(axum::middleware::from_fn_with_state(
                                    arc_secret_key.clone(),
                                    middleware::verify_token_middleware,
                                )),
                        )
                        .nest(
                            "/user/word/relation",
                            Router::new()
                                .route(
                                    "/user/:user_id/word/:user_word_id",
                                    get(|state, token, path| async move {
                                        Self::get_user_word_by_user_id_and_word_id(
                                            state, token, path,
                                        )
                                        .await
                                    }),
                                )
                                .route(
                                    "/user/:user_id",
                                    get(|state, token, path| async move {
                                        Self::get_user_word_by_user_id(state, token, path).await
                                    }),
                                )
                                .route(
                                    "/word/:word_id",
                                    get(|state, token, path| async move {
                                        Self::get_user_word_by_word_id(state, token, path).await
                                    }),
                                )
                                .route(
                                    "/",
                                    post(|state, token, body| async move {
                                        Self::create_user_word(state, token, body).await
                                    }),
                                )
                                .route(
                                    "/:user_word_id",
                                    delete(|state, token, path| async move {
                                        Self::delete_user_word(state, token, path).await
                                    }),
                                )
                                .route_layer(axum::middleware::from_fn_with_state(
                                    arc_secret_key.clone(),
                                    middleware::verify_token_middleware,
                                )),
                        )
                        .layer(axum::middleware::from_fn(
                            middleware::request_log_middleware,
                        )),
                )
                .with_state(self.service.clone());

        Ok(router)
    }

    async fn health_check(
        State(_): State<CosanService<U, W, UW>>,
    ) -> Result<(http::StatusCode, Json<response::HealthCheckResponse>), ()> {
        info!("Health check");

        Ok((
            http::StatusCode::OK,
            Json(response::HealthCheckResponse { status: "ok" }),
        ))
    }

    async fn get_user(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Path(user_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
        info!("Get user");
        info!(token = ?token);

        let protagonist = service.get_user(i64::try_from(user_id).unwrap()).await;
        match protagonist {
            Ok(protagonist) => Ok((http::StatusCode::OK, Json(protagonist))),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "User not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn create_user(
        State(service): State<CosanService<U, W, UW>>,
        Json(body): Json<request::CreateUserRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::CreateUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
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

        let user = service.create_user(body).await;
        match user {
            Ok(user) => Ok((http::StatusCode::CREATED, Json(user))),
            Err(err) => Err((
                http::StatusCode::CONFLICT,
                Json(response::ErrorResponse {
                    error: err.to_string(),
                    message: "User already exists".to_string(),
                }),
            )),
        }
    }

    async fn update_user(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Json(body): Json<request::UpdateUserRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::UpdateUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
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

        let protagonist = service.update_user(body).await;
        match protagonist {
            Ok(protagonist) => Ok((http::StatusCode::OK, Json(protagonist))),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(response::ErrorResponse {
                    error: err.to_string(),
                    message: "User not found".to_string(),
                }),
            )),
        }
    }

    async fn delete_user(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Path(protagonist_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::DeleteUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
        info!("Delete user");
        info!(token = ?token);

        let result = service
            .delete_user(i64::try_from(protagonist_id).unwrap())
            .await;

        match result {
            Ok(_) => Ok((
                http::StatusCode::OK,
                Json(response::DeleteUserResponse {
                    status: "The user has been deleted".to_string(),
                }),
            )),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(response::ErrorResponse {
                    error: err.to_string(),
                    message: "User not found".to_string(),
                }),
            )),
        }
    }

    async fn get_user_by_login_id_and_password(
        State(service): State<CosanService<U, W, UW>>,
        Path(login_request): Path<request::GetUserRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
        info!("Get user by login_id and password");

        let request = request::GetUserRequest::new(login_request.login_id, login_request.password)
            .validate()
            .await;
        if request.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(response::ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: request.err().unwrap().to_string(),
                }),
            ));
        }

        let user = service
            .get_user_by_login_id_and_password(request.unwrap())
            .await;

        match user {
            Ok(user) => Ok((http::StatusCode::OK, Json(user))),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "User not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn get_word(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Path(word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::GetWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
        info!("Get word");
        info!(token = ?token);

        let word = service.get_word(i64::try_from(word_id).unwrap()).await;

        match word {
            Ok(word) => Ok((http::StatusCode::OK, Json(word))),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "Word not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn create_word(
        State(service): State<CosanService<U, W, UW>>,
        Json(body): Json<request::CreateWordRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::CreateWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
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

        let word = service.create_word(body).await;
        match word {
            Ok(word) => Ok((http::StatusCode::CREATED, Json(word))),
            Err(err) => Err((
                http::StatusCode::CONFLICT,
                Json(response::ErrorResponse {
                    error: err.to_string(),
                    message: "Word already exists".to_string(),
                }),
            )),
        }
    }

    async fn update_word(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Json(body): Json<request::UpdateWordRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::UpdateWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
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

        let word = service.update_word(body).await;
        match word {
            Ok(word) => Ok((http::StatusCode::OK, Json(word))),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(response::ErrorResponse {
                    error: err.to_string(),
                    message: "Word not found".to_string(),
                }),
            )),
        }
    }

    async fn delete_word(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Path(word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::DeleteWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
        info!("Delete word");
        info!(token = ?token);

        let result = service.delete_word(i64::try_from(word_id).unwrap()).await;

        match result {
            Ok(_) => Ok((
                http::StatusCode::OK,
                Json(response::DeleteWordResponse {
                    status: "The word has been deleted".to_string(),
                }),
            )),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(response::ErrorResponse {
                    error: err.to_string(),
                    message: "Word not found".to_string(),
                }),
            )),
        }
    }

    async fn get_user_word_by_user_id_and_word_id(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Path(request): Path<request::GetUserWordRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
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

        let user_word = service.get_user_word_by_user_id_and_word_id(request).await;
        match user_word {
            Ok(user_word) => Ok((http::StatusCode::OK, Json(user_word))),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "User word not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn get_user_word_by_user_id(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Path(user_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<Vec<response::GetUserWordResponse>>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
        info!("Get user word");
        info!(token = ?token);

        let user_words = service
            .get_user_word_by_user_id(request::GetUserWordRequest {
                user_id: u64::try_from(user_id).unwrap(),
                word_id: 0,
            })
            .await;

        match user_words {
            Ok(user_words) => Ok((http::StatusCode::OK, Json(user_words))),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "User word not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn get_user_word_by_word_id(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Path(word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<Vec<response::GetUserWordResponse>>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
        info!("Get user word");
        info!(token = ?token);

        let user_words = service
            .get_user_word_by_word_id(request::GetUserWordRequest {
                user_id: 0,
                word_id: u64::try_from(word_id).unwrap(),
            })
            .await;

        match user_words {
            Ok(user_words) => Ok((http::StatusCode::OK, Json(user_words))),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "User word not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn create_user_word(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Json(body): Json<request::CreateUserWordRequest>,
    ) -> Result<
        (
            http::StatusCode,
            Json<response::CreateUserWordRelationResponse>,
        ),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
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

        let user_word = service.create_user_word(body).await;
        match user_word {
            Ok(user_word) => Ok((http::StatusCode::CREATED, Json(user_word))),
            Err(err) => Err((
                http::StatusCode::CONFLICT,
                Json(response::ErrorResponse {
                    error: err.to_string(),
                    message: "Protagonist supporter already exists".to_string(),
                }),
            )),
        }
    }

    async fn delete_user_word(
        State(service): State<CosanService<U, W, UW>>,
        Token(token): Token,
        Path(user_word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::DeleteUserWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    > {
        info!("Delete protagonist supporter");
        info!(token = ?token);

        let result = service
            .delete_user_word(i64::try_from(user_word_id).unwrap())
            .await;

        match result {
            Ok(_) => Ok((
                http::StatusCode::OK,
                Json(response::DeleteUserWordResponse {
                    status: "The protagonist supporter has been deleted".to_string(),
                }),
            )),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(response::ErrorResponse {
                    error: err.to_string(),
                    message: "Protagonist supporter not found".to_string(),
                }),
            )),
        }
    }
}
