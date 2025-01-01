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
use crate::driver::model;
use crate::router::request::GetSupporterRequest;
use axum::{
    http,
    routing::{delete, get, post, put},
    {
        extract::{Path, State},
        Json, Router, Server,
    },
};
use std::net::SocketAddr;
use tracing::info;

#[derive(Clone)]
pub struct AppRouter {
    service: SupportService,
}

impl AppRouter {
    pub fn new(service: SupportService) -> Self {
        Self { service }
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
        let router = Router::new()
            .nest(
                "/support/v1",
                Router::new()
                    .route("/health", get(Self::health_check))
                    .nest(
                        "/protagonist",
                        Router::new()
                            .route("/:protagonist_id", get(Self::get_protagonist))
                            .route("/", post(Self::create_protagonist))
                            .route("/", put(Self::update_protagonist))
                            .route("/:protagonist_id", delete(Self::delete_protagonist))
                            .route(
                                "/login/:login_id/password/:password",
                                get(Self::get_protagonist_by_login_id_and_password),
                            ),
                    )
                    .nest(
                        "/supporter",
                        Router::new()
                            .route("/:supporter_id", get(Self::get_supporter))
                            .route("/", post(Self::create_supporter))
                            .route("/", put(Self::update_supporter))
                            .route("/:supporter_id", delete(Self::delete_supporter))
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
                            ),
                    ),
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

    async fn get_protagonist(
        State(service): State<SupportService>,
        Path(protagonist_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<GetProtagonistResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Get protagonist");

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

        // protagonist_id is set to -1 because it is auto-incremented in the database
        let protagonist = service
            .create_protagonist(model::CreateProtagonist::new(
                -1,
                body.last_name,
                body.first_name,
                body.login_id,
                body.password,
                body.email,
                body.country,
            ))
            .await;

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
        Json(body): Json<UpdateProtagonistRequest>,
    ) -> Result<
        (http::StatusCode, Json<UpdateProtagonistResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Update protagonist");

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

        let protagonist = service
            .update_protagonist(model::UpdateProtagonist::new(
                body.protagonist_id,
                body.last_name,
                body.first_name,
                body.login_id,
                body.password,
                body.email,
                body.country,
            ))
            .await;

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
        Path(protagonist_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<DeleteProtagonistResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Delete protagonist");

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
        Path(supporter_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<GetSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Get supporter");

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

        // support_id is set to -1 because it is auto-incremented in the database
        let supporter = service
            .create_supporter(model::CreateSupporter::new(
                -1,
                body.last_name,
                body.first_name,
                body.login_id,
                body.password,
                body.email,
                body.country,
            ))
            .await;

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
        Json(body): Json<UpdateSupporterRequest>,
    ) -> Result<
        (http::StatusCode, Json<UpdateSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Update supporter");

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

        let supporter = service
            .update_supporter(model::UpdateSupporter::new(
                body.supporter_id,
                body.last_name,
                body.first_name,
                body.login_id,
                body.password,
                body.email,
                body.country,
            ))
            .await;

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
        Path(supporter_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<DeleteSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Delete supporter");

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
        Path(protagonist_supporter_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<Vec<GetProtagonistSupporterResponse>>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Get protagonist supporter");

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
        Json(body): Json<CreateProtagonistSupporterRequest>,
    ) -> Result<
        (http::StatusCode, Json<CreateProtagonistSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Create protagonist supporter");

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

        let protagonist_supporter = service
            .create_protagonist_supporter(model::CreateProtagonistSupporter {
                protagonist_supporter_id: -1,
                protagonist_id: i64::try_from(body.protagonist_id).unwrap(),
                supporter_id: i64::try_from(body.supporter_id).unwrap(),
            })
            .await;

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
        Path(protagonist_supporter_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<DeleteProtagonistSupporterResponse>),
        (http::StatusCode, Json<ErrorResponse>),
    > {
        info!("Delete protagonist supporter");

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
