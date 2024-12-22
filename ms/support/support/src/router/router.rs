use axum::{
    routing::{delete, get, post, put},
    Router,
    Server,
    http::StatusCode,
    Json,
};
use std::net::SocketAddr;
use tracing::info;

use super::response::{HealthCheckResponse};

pub async fn new_router() -> Result<(), anyhow::Error> {
    let router = Router::new()
        .route("/health", get(health_check))
        .route("/protagonist",
            get(get_protagonist)
            .post(create_protagonist)
            .put(update_protagonist)
            .delete(delete_protagonist)
        ).route("/supporter",
            get(get_supporter)
            .post(create_supporter)
            .put(update_supporter)
            .delete(delete_supporter)
        ).route("/protagonist_supporter",
            get(get_protagonist_supporter)
            .post(create_protagonist_supporter)
            .put(update_protagonist_supporter)
            .delete(delete_protagonist_supporter)
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;

    Ok(())
}

async fn health_check() -> Result<Json<HealthCheckResponse>, StatusCode> {
    info!("Health check");
    Ok(Json(HealthCheckResponse { status: "ok" }))
}

async fn get_protagonist() -> Result<&'static str, StatusCode> {
    info!("Get protagonist");
    Ok("The protagonist is a person")
}

async fn create_protagonist() -> Result<&'static str, StatusCode> {
    info!("Create protagonist");
    Ok("The protagonist has been created")
}

async fn update_protagonist() -> Result<&'static str, StatusCode> {
    info!("Update protagonist");
    Ok("The protagonist has been updated")
}

async fn delete_protagonist() -> Result<&'static str, StatusCode> {
    info!("Delete protagonist");
    Ok("The protagonist has been deleted")
}

async fn get_supporter() -> Result<&'static str, StatusCode> {
    info!("Get supporter");
    Ok("The supporter is a person")
}

async fn create_supporter() -> Result<&'static str, StatusCode> {
    info!("Create supporter");
    Ok("The supporter has been created")
}

async fn update_supporter() -> Result<&'static str, StatusCode> {
    info!("Update supporter");
    Ok("The supporter has been updated")
}

async fn delete_supporter() -> Result<&'static str, StatusCode> {
    info!("Delete supporter");
    Ok("The supporter has been deleted")
}

async fn get_protagonist_supporter() -> Result<&'static str, StatusCode> {
    info!("Get protagonist supporter");
    Ok("The protagonist supporter is a person")
}

async fn create_protagonist_supporter() -> Result<&'static str, StatusCode> {
    info!("Create protagonist supporter");
    Ok("The protagonist supporter has been created")
}

async fn update_protagonist_supporter() -> Result<&'static str, StatusCode> {
    info!("Update protagonist supporter");
    Ok("The protagonist supporter has been updated")
}

async fn delete_protagonist_supporter() -> Result<&'static str, StatusCode> {
    info!("Delete protagonist supporter");
    Ok("The protagonist supporter has been deleted")
}