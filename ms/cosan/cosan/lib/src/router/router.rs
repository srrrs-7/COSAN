use super::{
    middleware,
    request::{
        CreateProtagonistSupporterRequest, CreateSupporterRequest, CreateUserRequest,
        GetUserRequest, UpdateSupporterRequest, UpdateUserRequest,
    },
    response::{
        CreateProtagonistSupporterResponse, CreateSupporterResponse, CreateUserResponse,
        DeleteProtagonistSupporterResponse, DeleteSupporterResponse, DeleteUserResponse,
        ErrorResponse, GetProtagonistSupporterResponse, GetSupporterResponse, GetUserResponse,
        HealthCheckResponse, UpdateSupporterResponse, UpdateUserResponse,
    },
};
use crate::{domain::service::SupportService, util};
use axum::{
    http,
    routing::{delete, get, post, put},
    {
        extract::{Extension, Path, State},
        Json, Router, Server,
    },
};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

#[derive(Clone)]
pub struct AppRouter {
    service: SupportService,
    secret_key: String,
}

impl AppRouter {
    pub fn new(service: SupportService, secret_key: String) -> Self {
        Self {
            service,
            secret_key,
        }
    }

    pub async fn serve(&self) -> Result<(), anyhow::Error> {
        let router = Self::init_router(self).await?;
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        info!("Listening on {}", addr);

        Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;

        Ok(())
    }

    async fn init_router(&self) -> Result<Router, anyhow::Error> {
        let arc_secret_key = Arc::new(self.secret_key.clone());

        let router = Router::new()
            .nest(
                "/cosan/v1",
                Router::new()
                    .route("/health", get(Self::health_check))
                    .nest(
                        "/user",
                        Router::new()
                            .route("/:user_id", get(Self::get_user))
                            .route("/", put(Self::update_user))
                            .route("/:user_id", delete(Self::delete_user))
                            .route_layer(axum::middleware::from_fn_with_state(
                                arc_secret_key.clone(),
                                middleware::verify_token_middleware,
                            ))
                            .route("/", post(Self::create_user))
                            .route(
                                "/login/:login_id/password/:password",
                                get(Self::get_user_by_login_id_and_password),
                            ),
                    )
                    .nest(
                        "/word",
                        Router::new()
                            .route("/", post(Self::create_word))
                            .route("/:word_id", get(Self::get_word))
                            .route("/", put(Self::update_word))
                            .route("/:word_id", delete(Self::delete_word))
                            .route_layer(axum::middleware::from_fn_with_state(
                                arc_secret_key.clone(),
                                middleware::verify_token_middleware,
                            )),
                    )
                    .nest(
                        "/user/word",
                        Router::new()
                            .route("/:user_word_id", get(Self::get_user_word))
                            .route("/", post(Self::create_user_word))
                            .route("/:user_word_id", delete(Self::delete_user_word))
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
        State(_): State<SupportService>,
    ) -> Result<(http::StatusCode, Json<HealthCheckResponse>), ()> {
        info!("Health check");

        Ok((
            http::StatusCode::OK,
            Json(HealthCheckResponse { status: "ok" }),
        ))
    }

    async fn get_user(
        Extension(token): Extension<Arc<util::auth::Token>>,
        State(service): State<SupportService>,
        Path(user_id): Path<u64>,
    ) -> Result<(http::StatusCode, Json<GetUserResponse>), (http::StatusCode, Json<ErrorResponse>)>
    {
        info!("Get user");
        info!(token = ?token);

        let protagonist = service.get_user(i64::try_from(user_id).unwrap()).await;

        match protagonist {
            Ok(protagonist) => Ok((http::StatusCode::OK, Json(protagonist))),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(ErrorResponse {
                            error: err.to_string(),
                            message: "Protagonist not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn create_user(
        State(service): State<SupportService>,
        Json(body): Json<CreateUserRequest>,
    ) -> Result<(http::StatusCode, Json<CreateUserResponse>), (http::StatusCode, Json<ErrorResponse>)>
    {
        info!("Create user");

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
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
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "User already exists".to_string(),
                }),
            )),
        }
    }

    async fn update_user(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Json(body): Json<UpdateUserRequest>,
    ) -> Result<(http::StatusCode, Json<UpdateUserResponse>), (http::StatusCode, Json<ErrorResponse>)>
    {
        info!("Update user");
        info!(token = ?token);

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
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
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "User not found".to_string(),
                }),
            )),
        }
    }

    async fn delete_user(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(protagonist_id): Path<u64>,
    ) -> Result<(http::StatusCode, Json<DeleteUserResponse>), (http::StatusCode, Json<ErrorResponse>)>
    {
        info!("Delete user");
        info!(token = ?token);

        let result = service
            .delete_user(i64::try_from(protagonist_id).unwrap())
            .await;

        match result {
            Ok(_) => Ok((
                http::StatusCode::OK,
                Json(DeleteUserResponse {
                    status: "The user has been deleted".to_string(),
                }),
            )),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "User not found".to_string(),
                }),
            )),
        }
    }

    async fn get_user_by_login_id_and_password(
        State(service): State<SupportService>,
        Path(login_request): Path<GetUserRequest>,
    ) -> Result<(http::StatusCode, Json<GetUserResponse>), (http::StatusCode, Json<ErrorResponse>)>
    {
        info!("Get protagonist by login_id and password");

        let request = GetUserRequest::new(login_request.login_id, login_request.password)
            .validate()
            .await;
        if request.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
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
                        Json(ErrorResponse {
                            error: err.to_string(),
                            message: "User not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn get_word(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(supporter_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<GetSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Get supporter");
        info!(token = ?token);

        let supporter = service
            .get_supporter(i64::try_from(supporter_id).unwrap())
            .await;

        match supporter {
            Ok(supporter) => Ok((http::StatusCode::OK, Json(supporter))),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(ErrorResponse {
                            error: err.to_string(),
                            message: "Supporter not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn create_word(
        State(service): State<SupportService>,
        Json(body): Json<CreateSupporterRequest>,
    ) -> Result<
        (http::StatusCode, Json<CreateSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Create supporter");

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: valid.err().unwrap().to_string(),
                }),
            ));
        }

        let supporter = service.create_supporter(body).await;
        match supporter {
            Ok(supporter) => Ok((http::StatusCode::CREATED, Json(supporter))),
            Err(err) => Err((
                http::StatusCode::CONFLICT,
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "Supporter already exists".to_string(),
                }),
            )),
        }
    }

    async fn update_word(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Json(body): Json<UpdateSupporterRequest>,
    ) -> Result<
        (http::StatusCode, Json<UpdateSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Update supporter");
        info!(token = ?token);

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: valid.err().unwrap().to_string(),
                }),
            ));
        }

        let supporter = service.update_supporter(body).await;
        match supporter {
            Ok(supporter) => Ok((http::StatusCode::OK, Json(supporter))),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "Supporter not found".to_string(),
                }),
            )),
        }
    }

    async fn delete_word(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(supporter_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<DeleteSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Delete supporter");
        info!(token = ?token);

        let result = service
            .delete_supporter(i64::try_from(supporter_id).unwrap())
            .await;

        match result {
            Ok(_) => Ok((
                http::StatusCode::OK,
                Json(DeleteSupporterResponse {
                    status: "The supporter has been deleted".to_string(),
                }),
            )),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "Supporter not found".to_string(),
                }),
            )),
        }
    }

    async fn get_user_word(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(protagonist_supporter_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<Vec<GetProtagonistSupporterResponse>>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Get protagonist supporter");
        info!(token = ?token);

        let protagonist_supporters = service
            .get_protagonist_supporter(i64::try_from(protagonist_supporter_id).unwrap())
            .await;

        match protagonist_supporters {
            Ok(protagonist_supporters) => Ok((
                http::StatusCode::OK,
                Json(protagonist_supporters.into_iter().collect()),
            )),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(ErrorResponse {
                            error: err.to_string(),
                            message: "Protagonist supporter not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }

    async fn create_user_word(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Json(body): Json<CreateProtagonistSupporterRequest>,
    ) -> Result<
        (http::StatusCode, Json<CreateProtagonistSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Create protagonist supporter");
        info!(token = ?token);

        let valid = body.validate().await;
        if valid.is_err() {
            return Err((
                http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: valid.err().unwrap().to_string(),
                }),
            ));
        }

        let protagonist_supporter = service.create_protagonist_supporter(body).await;
        match protagonist_supporter {
            Ok(protagonist_supporter) => {
                Ok((http::StatusCode::CREATED, Json(protagonist_supporter)))
            }
            Err(err) => Err((
                http::StatusCode::CONFLICT,
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "Protagonist supporter already exists".to_string(),
                }),
            )),
        }
    }

    async fn delete_user_word(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(protagonist_supporter_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<DeleteProtagonistSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Delete protagonist supporter");
        info!(token = ?token);

        let result = service
            .delete_protagonist_supporter(i64::try_from(protagonist_supporter_id).unwrap())
            .await;

        match result {
            Ok(_) => Ok((
                http::StatusCode::OK,
                Json(DeleteProtagonistSupporterResponse {
                    status: "The protagonist supporter has been deleted".to_string(),
                }),
            )),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "Protagonist supporter not found".to_string(),
                }),
            )),
        }
    }
}
