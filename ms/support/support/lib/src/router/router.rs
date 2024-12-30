use super::request::{
    CreateProtagonistRequest, CreateProtagonistSupporterRequest, CreateSupporterRequest,
    UpdateProtagonistRequest, UpdateSupporterRequest,
};
use super::response::{
    CreateProtagonistResponse, CreateProtagonistSupporterResponse, CreateSupporterResponse,
    DeleteProtagonistResponse, DeleteProtagonistSupporterResponse, DeleteSupporterResponse,
    ErrorResponse, GetProtagonistResponse, GetProtagonistSupporterResponse, GetSupporterResponse,
    HealthCheckResponse, UpdateProtagonistResponse, UpdateSupporterResponse,
};
use crate::domain::service::SupportService;
use crate::driver::model;
use axum::{
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
            .route("/health", get(Self::health_check))
            .nest(
                "/protagonist",
                Router::new()
                    .route("/:protagonist_id", get(Self::get_protagonist))
                    .route("/", post(Self::create_protagonist))
                    .route("/", put(Self::update_protagonist))
                    .route("/:protagonist_id", delete(Self::delete_protagonist)),
            )
            .nest(
                "/supporter",
                Router::new()
                    .route("/:supporter_id", get(Self::get_supporter))
                    .route("/", post(Self::create_supporter))
                    .route("/", put(Self::update_supporter))
                    .route("/:supporter_id", delete(Self::delete_supporter)),
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
            )
            .with_state(self.service.clone());

        Ok(router)
    }

    async fn health_check(Path(id): Path<u64>) -> Result<Json<HealthCheckResponse>, ()> {
        info!("Health check");
        Ok(Json(HealthCheckResponse { id, status: "ok" }))
    }

    async fn get_protagonist(
        State(service): State<SupportService>,
        Path(protagonist_id): Path<u64>,
    ) -> Result<Json<GetProtagonistResponse>, Json<ErrorResponse>> {
        info!("Get protagonist");

        let protagonist = service
            .get_protagonist(i64::try_from(protagonist_id).unwrap())
            .await;

        match protagonist {
            Ok(protagonist) => Ok(Json(protagonist)),
            Err(err) => {
                return Err(Json(ErrorResponse {
                    error: err.to_string(),
                    message: "Protagonist already exists".to_string(),
                }));
            }
        }
    }

    async fn create_protagonist(
        State(service): State<SupportService>,
        Json(body): Json<CreateProtagonistRequest>,
    ) -> Result<Json<CreateProtagonistResponse>, Json<ErrorResponse>> {
        info!("Create protagonist");

        let valid = body.validate().await;
        if valid.is_err() {
            return Err(Json(ErrorResponse {
                error: "Bad Request".to_string(),
                message: valid.err().unwrap().to_string(),
            }));
        }

        let protagonist = service
            .create_protagonist(model::CreateProtagonist {
                protagonist_id: -1,
                last_name: body.last_name,
                first_name: body.first_name,
                email: body.email,
                country: body.country,
            })
            .await;

        match protagonist {
            Ok(protagonist) => Ok(Json(protagonist)),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Protagonist already exists".to_string(),
            })),
        }
    }

    async fn update_protagonist(
        State(service): State<SupportService>,
        Json(body): Json<UpdateProtagonistRequest>,
    ) -> Result<Json<UpdateProtagonistResponse>, Json<ErrorResponse>> {
        info!("Update protagonist");

        let valid = body.validate().await;
        if valid.is_err() {
            return Err(Json(ErrorResponse {
                error: "Bad Request".to_string(),
                message: valid.err().unwrap().to_string(),
            }));
        }

        let protagonist = service
            .update_protagonist(model::UpdateProtagonist {
                protagonist_id: body.protagonist_id,
                last_name: body.last_name,
                first_name: body.first_name,
                email: body.email,
                country: body.country,
            })
            .await;

        match protagonist {
            Ok(protagonist) => Ok(Json(protagonist)),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Protagonist already exists".to_string(),
            })),
        }
    }

    async fn delete_protagonist(
        State(service): State<SupportService>,
        Path(protagonist_id): Path<u64>,
    ) -> Result<Json<DeleteProtagonistResponse>, Json<ErrorResponse>> {
        info!("Delete protagonist");

        let result = service
            .delete_protagonist(i64::try_from(protagonist_id).unwrap())
            .await;

        match result {
            Ok(_) => Ok(Json(DeleteProtagonistResponse {
                status: "The protagonist has been deleted".to_string(),
            })),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Protagonist not found".to_string(),
            })),
        }
    }

    async fn get_supporter(
        State(service): State<SupportService>,
        Path(supporter_id): Path<u64>,
    ) -> Result<Json<GetSupporterResponse>, Json<ErrorResponse>> {
        info!("Get supporter");

        let supporter = service
            .get_supporter(i64::try_from(supporter_id).unwrap())
            .await;

        match supporter {
            Ok(supporter) => Ok(Json(supporter)),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Supporter not found".to_string(),
            })),
        }
    }

    async fn create_supporter(
        State(service): State<SupportService>,
        Json(body): Json<CreateSupporterRequest>,
    ) -> Result<Json<CreateSupporterResponse>, Json<ErrorResponse>> {
        info!("Create supporter");

        let valid = body.validate().await;
        if valid.is_err() {
            return Err(Json(ErrorResponse {
                error: "Bad Request".to_string(),
                message: valid.err().unwrap().to_string(),
            }));
        }

        let supporter = service
            .create_supporter(model::CreateSupporter {
                supporter_id: -1,
                last_name: body.last_name,
                first_name: body.first_name,
                email: body.email,
                country: body.country,
            })
            .await;

        match supporter {
            Ok(supporter) => Ok(Json(supporter)),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Supporter already exists".to_string(),
            })),
        }
    }

    async fn update_supporter(
        State(service): State<SupportService>,
        Json(body): Json<UpdateSupporterRequest>,
    ) -> Result<Json<UpdateSupporterResponse>, Json<ErrorResponse>> {
        info!("Update supporter");

        let valid = body.validate().await;
        if valid.is_err() {
            return Err(Json(ErrorResponse {
                error: "Bad Request".to_string(),
                message: valid.err().unwrap().to_string(),
            }));
        }

        let supporter = service
            .update_supporter(model::UpdateSupporter {
                supporter_id: body.supporter_id,
                last_name: body.last_name,
                first_name: body.first_name,
                email: body.email,
                country: body.country,
            })
            .await;

        match supporter {
            Ok(supporter) => Ok(Json(supporter)),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Supporter already exists".to_string(),
            })),
        }
    }

    async fn delete_supporter(
        State(service): State<SupportService>,
        Path(supporter_id): Path<u64>,
    ) -> Result<Json<DeleteSupporterResponse>, Json<ErrorResponse>> {
        info!("Delete supporter");

        let result = service
            .delete_supporter(i64::try_from(supporter_id).unwrap())
            .await;

        match result {
            Ok(_) => Ok(Json(DeleteSupporterResponse {
                status: "The supporter has been deleted".to_string(),
            })),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Supporter not found".to_string(),
            })),
        }
    }

    async fn get_protagonist_supporter(
        State(service): State<SupportService>,
        Path(protagonist_supporter_id): Path<u64>,
    ) -> Result<Json<GetProtagonistSupporterResponse>, Json<ErrorResponse>> {
        info!("Get protagonist supporter");

        let protagonist_supporter = service
            .get_protagonist_supporter(i64::try_from(protagonist_supporter_id).unwrap())
            .await;

        match protagonist_supporter {
            Ok(protagonist_supporter) => Ok(Json(protagonist_supporter)),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Protagonist supporter not found".to_string(),
            })),
        }
    }

    async fn create_protagonist_supporter(
        State(service): State<SupportService>,
        Json(body): Json<CreateProtagonistSupporterRequest>,
    ) -> Result<Json<CreateProtagonistSupporterResponse>, Json<ErrorResponse>> {
        info!("Create protagonist supporter");

        let valid = body.validate().await;
        if valid.is_err() {
            return Err(Json(ErrorResponse {
                error: "Bad Request".to_string(),
                message: valid.err().unwrap().to_string(),
            }));
        }

        let protagonist_supporter = service
            .create_protagonist_supporter(model::CreateProtagonistSupporter {
                protagonist_id: i64::try_from(body.protagonist_id).unwrap(),
                supporter_id: i64::try_from(body.supporter_id).unwrap(),
            })
            .await;

        match protagonist_supporter {
            Ok(protagonist_supporter) => Ok(Json(protagonist_supporter)),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Protagonist supporter already exists".to_string(),
            })),
        }
    }

    async fn delete_protagonist_supporter(
        State(service): State<SupportService>,
        Path(protagonist_supporter_id): Path<u64>,
    ) -> Result<Json<DeleteProtagonistSupporterResponse>, Json<ErrorResponse>> {
        info!("Delete protagonist supporter");

        let result = service
            .delete_protagonist_supporter(i64::try_from(protagonist_supporter_id).unwrap())
            .await;

        match result {
            Ok(_) => Ok(Json(DeleteProtagonistSupporterResponse {
                status: "The protagonist supporter relation has been deleted".to_string(),
            })),
            Err(err) => Err(Json(ErrorResponse {
                error: err.to_string(),
                message: "Supporter not found".to_string(),
            })),
        }
    }
}
