use super::request::{
    CreateProtagonistRequest, CreateProtagonistSupporterRequest, CreateSupporterRequest,
    GetProtagonistRequest, UpdateProtagonistRequest, UpdateSupporterRequest,
};
use super::response::{
    CreateProtagonistResponse, CreateProtagonistSupporterResponse, CreateSupporterResponse,
    DeleteProtagonistResponse, DeleteProtagonistSupporterResponse, DeleteSupporterResponse,
    ErrorResponse, GetProtagonistResponse, GetProtagonistSupporterResponse, GetSupporterResponse,
    HealthCheckResponse, UpdateProtagonistResponse, UpdateSupporterResponse,
};
use crate::domain::service::SupportService;
use crate::router::request::GetSupporterRequest;
use crate::util;
use axum::middleware;
use axum::{
    http,
    middleware::Next,
    response::Response,
    routing::{delete, get, post, put},
    {
        extract::{Extension, Path, State},
        Json, Router, Server,
    },
};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};

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
                "/support/v1",
                Router::new()
                    .route("/health", get(Self::health_check))
                    .nest(
                        "/protagonist",
                        Router::new()
                            .route("/:protagonist_id", get(Self::get_protagonist))
                            .route("/", put(Self::update_protagonist))
                            .route("/:protagonist_id", delete(Self::delete_protagonist))
                            .route_layer(middleware::from_fn_with_state(
                                arc_secret_key.clone(),
                                Self::verify_token_middleware,
                            ))
                            .route("/", post(Self::create_protagonist))
                            .route(
                                "/login/:login_id/password/:password",
                                get(Self::get_protagonist_by_login_id_and_password),
                            ),
                    )
                    .nest(
                        "/supporter",
                        Router::new()
                            .route("/:supporter_id", get(Self::get_supporter))
                            .route("/", put(Self::update_supporter))
                            .route("/:supporter_id", delete(Self::delete_supporter))
                            .route_layer(middleware::from_fn_with_state(
                                arc_secret_key.clone(),
                                Self::verify_token_middleware,
                            ))
                            .route("/", post(Self::create_supporter))
                            .route(
                                "/login/:login_id/password/:password",
                                get(Self::get_supporter_by_login_id_and_password),
                            ),
                    )
                    .nest(
                        "/protagonist_supporter",
                        Router::new()
                            .route(
                                "/:protagonist_supporter_id",
                                get(Self::get_protagonist_supporter),
                            )
                            .route("/", post(Self::create_protagonist_supporter))
                            .route(
                                "/:protagonist_supporter_id",
                                delete(Self::delete_protagonist_supporter),
                            )
                            .route_layer(middleware::from_fn_with_state(
                                arc_secret_key.clone(),
                                Self::verify_token_middleware,
                            )),
                    )
                    .layer(axum::middleware::from_fn(Self::request_log_middleware)),
            )
            .with_state(self.service.clone());

        Ok(router)
    }

    async fn verify_token_middleware<B>(
        State(secret_key): State<Arc<String>>,
        mut req: http::Request<B>,
        next: Next<B>,
    ) -> Result<Response, http::StatusCode> {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let auth_header = if let Some(auth_header) = auth_header {
            auth_header
        } else {
            error!("verify_token_middleware: Authorization header not found");
            return Err(http::StatusCode::UNAUTHORIZED);
        };

        let bearer = auth_header.split_whitespace().nth(0).unwrap_or_default();
        if bearer != "Bearer" {
            error!("verify_token_middleware: Authorization header is not Bearer");
            return Err(http::StatusCode::UNAUTHORIZED);
        }

        let token = auth_header.split_whitespace().nth(1).unwrap_or_default();
        if token.is_empty() {
            error!("verify_token_middleware: Token is empty");
            return Err(http::StatusCode::UNAUTHORIZED);
        }

        let token = util::auth::validate_token(token, &secret_key).map_err(|e| {
            error!("verify_token_middleware: {}", e);
            return http::StatusCode::UNAUTHORIZED;
        });
        if token.is_err() {
            return Err(http::StatusCode::UNAUTHORIZED);
        }

        // token info add to request context
        req.extensions_mut().insert(Arc::new(token.unwrap()));

        Ok(next.run(req).await)
    }

    async fn request_log_middleware<B>(
        req: http::Request<B>,
        next: Next<B>,
    ) -> Result<Response, http::StatusCode> {
        info!("Request: {} {}", req.method(), req.uri());
        let res = next.run(req).await;
        info!("Response: {}", res.status());

        Ok(res)
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

    async fn get_protagonist(
        Extension(token): Extension<Arc<util::auth::Token>>,
        State(service): State<SupportService>,
        Path(protagonist_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<GetProtagonistResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Get protagonist");
        info!(token = ?token);

        let protagonist = service
            .get_protagonist(i64::try_from(protagonist_id).unwrap())
            .await;

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

    async fn create_protagonist(
        State(service): State<SupportService>,
        Json(body): Json<CreateProtagonistRequest>,
    ) -> Result<
        (http::StatusCode, Json<CreateProtagonistResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Create protagonist");

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

        let protagonist = service.create_protagonist(body).await;
        match protagonist {
            Ok(protagonist) => Ok((http::StatusCode::CREATED, Json(protagonist))),
            Err(err) => Err((
                http::StatusCode::CONFLICT,
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "Protagonist already exists".to_string(),
                }),
            )),
        }
    }

    async fn update_protagonist(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Json(body): Json<UpdateProtagonistRequest>,
    ) -> Result<
        (http::StatusCode, Json<UpdateProtagonistResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Update protagonist");
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

        let protagonist = service.update_protagonist(body).await;
        match protagonist {
            Ok(protagonist) => Ok((http::StatusCode::OK, Json(protagonist))),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "Protagonist not found".to_string(),
                }),
            )),
        }
    }

    async fn delete_protagonist(
        State(service): State<SupportService>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(protagonist_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<DeleteProtagonistResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Delete protagonist");
        info!(token = ?token);

        let result = service
            .delete_protagonist(i64::try_from(protagonist_id).unwrap())
            .await;

        match result {
            Ok(_) => Ok((
                http::StatusCode::OK,
                Json(DeleteProtagonistResponse {
                    status: "The protagonist has been deleted".to_string(),
                }),
            )),
            Err(err) => Err((
                http::StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: err.to_string(),
                    message: "Protagonist not found".to_string(),
                }),
            )),
        }
    }

    async fn get_protagonist_by_login_id_and_password(
        State(service): State<SupportService>,
        Path(login_request): Path<GetProtagonistRequest>,
    ) -> Result<
        (http::StatusCode, Json<GetProtagonistResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Get protagonist by login_id and password");

        let request = GetProtagonistRequest::new(login_request.login_id, login_request.password)
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

        let protagonist = service
            .get_protagonist_by_login_id_and_password(request.unwrap())
            .await;

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

    async fn get_supporter(
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

    async fn create_supporter(
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

    async fn update_supporter(
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

    async fn delete_supporter(
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

    async fn get_supporter_by_login_id_and_password(
        State(service): State<SupportService>,
        Path(login_request): Path<GetSupporterRequest>,
    ) -> Result<
        (http::StatusCode, Json<GetSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Get supporter by login_id and password");

        let request = GetSupporterRequest::new(login_request.login_id, login_request.password)
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

        let supporter = service
            .get_supporter_by_login_id_and_password(request.unwrap())
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

    async fn get_protagonist_supporter(
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

    async fn create_protagonist_supporter(
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

    async fn delete_protagonist_supporter(
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
