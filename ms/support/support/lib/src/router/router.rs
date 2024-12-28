use super::response::{
    CreateProtagonistResponse, CreateProtagonistSupporterResponse, CreateSupporterResponse,
    DeleteProtagonistResponse, DeleteProtagonistSupporterResponse, DeleteSupporterResponse,
    GetProtagonistResponse, GetProtagonistSupporterResponse, GetSupporterResponse,
    HealthCheckResponse, UpdateProtagonistResponse, UpdateProtagonistSupporterResponse,
    UpdateSupporterResponse,
};
use axum::{http::StatusCode, routing::get, Json, Router, Server};
use std::net::SocketAddr;
use tracing::info;

pub struct AppRouter {}

impl AppRouter {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn serve(&self) -> Result<(), anyhow::Error> {
        let router = Router::new()
            .route("/health", get(health_check))
            .nest(
                "/protagonist",
                Router::new().route(
                    "/",
                    get(get_protagonist)
                        .post(create_protagonist)
                        .put(update_protagonist)
                        .delete(delete_protagonist),
                ),
            )
            .nest(
                "/supporter",
                Router::new().route(
                    "/",
                    get(get_supporter)
                        .post(create_supporter)
                        .put(update_supporter)
                        .delete(delete_supporter),
                ),
            )
            .nest(
                "/protagonist_supporter",
                Router::new().route(
                    "/",
                    get(get_protagonist_supporter)
                        .post(create_protagonist_supporter)
                        .put(update_protagonist_supporter)
                        .delete(delete_protagonist_supporter),
                ),
            );

        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        info!("Listening on {}", addr);

        Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;

        Ok(())
    }
}

async fn health_check() -> Result<Json<HealthCheckResponse>, StatusCode> {
    info!("Health check");
    Ok(Json(HealthCheckResponse { status: "ok" }))
}

async fn get_protagonist() -> Result<Json<GetProtagonistResponse>, StatusCode> {
    info!("Get protagonist");
    Ok(Json(GetProtagonistResponse {
        id: 1,
        name: "Alice".to_string(),
    }))
}

async fn create_protagonist() -> Result<Json<CreateProtagonistResponse>, StatusCode> {
    info!("Create protagonist");
    Ok(Json(CreateProtagonistResponse {
        status: "The protagonist has been created".to_string(),
    }))
}

async fn update_protagonist() -> Result<Json<UpdateProtagonistResponse>, StatusCode> {
    info!("Update protagonist");
    Ok(Json(UpdateProtagonistResponse {
        status: "The protagonist has been updated".to_string(),
    }))
}

async fn delete_protagonist() -> Result<Json<DeleteProtagonistResponse>, StatusCode> {
    info!("Delete protagonist");
    Ok(Json(DeleteProtagonistResponse {
        status: "The protagonist has been deleted".to_string(),
    }))
}

async fn get_supporter() -> Result<Json<GetSupporterResponse>, StatusCode> {
    info!("Get supporter");
    Ok(Json(GetSupporterResponse {
        id: 1,
        name: "Bob".to_string(),
    }))
}

async fn create_supporter() -> Result<Json<CreateSupporterResponse>, StatusCode> {
    info!("Create supporter");
    Ok(Json(CreateSupporterResponse {
        status: "The supporter has been created".to_string(),
    }))
}

async fn update_supporter() -> Result<Json<UpdateSupporterResponse>, StatusCode> {
    info!("Update supporter");
    Ok(Json(UpdateSupporterResponse {
        status: "The supporter has been updated".to_string(),
    }))
}

async fn delete_supporter() -> Result<Json<DeleteSupporterResponse>, StatusCode> {
    info!("Delete supporter");
    Ok(Json(DeleteSupporterResponse {
        status: "The supporter has been deleted".to_string(),
    }))
}

async fn get_protagonist_supporter() -> Result<Json<GetProtagonistSupporterResponse>, StatusCode> {
    info!("Get protagonist supporter");
    Ok(Json(GetProtagonistSupporterResponse {
        protagonist_id: 1,
        protagonist_name: "Alice".to_string(),
        supporter_id: 1,
        supporter_name: "Bob".to_string(),
    }))
}

async fn create_protagonist_supporter(
) -> Result<Json<CreateProtagonistSupporterResponse>, StatusCode> {
    info!("Create protagonist supporter");
    Ok(Json(CreateProtagonistSupporterResponse {
        status: "The protagonist supporter has been created".to_string(),
    }))
}

async fn update_protagonist_supporter(
) -> Result<Json<UpdateProtagonistSupporterResponse>, StatusCode> {
    info!("Update protagonist supporter");
    Ok(Json(UpdateProtagonistSupporterResponse {
        status: "The protagonist supporter has been updated".to_string(),
    }))
}

async fn delete_protagonist_supporter(
) -> Result<Json<DeleteProtagonistSupporterResponse>, StatusCode> {
    info!("Delete protagonist supporter");
    Ok(Json(DeleteProtagonistSupporterResponse {
        status: "The protagonist supporter has been deleted".to_string(),
    }))
}
