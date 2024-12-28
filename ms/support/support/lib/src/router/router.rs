use super::request::{
    CreateProtagonistRequest, CreateProtagonistSupporterRequest, CreateSupporterRequest,
    GetProtagonistRequest, GetSupporterRequest, UpdateProtagonistRequest,
    UpdateProtagonistSupporterRequest, UpdateSupporterRequest,
};
use super::response::{
    CreateProtagonistResponse, CreateProtagonistSupporterResponse, CreateSupporterResponse,
    DeleteProtagonistResponse, DeleteProtagonistSupporterResponse, DeleteSupporterResponse,
    GetProtagonistResponse, GetProtagonistSupporterResponse, GetSupporterResponse,
    HealthCheckResponse, UpdateProtagonistResponse, UpdateProtagonistSupporterResponse,
    UpdateSupporterResponse,
};
use axum::extract::Query;
use axum::routing::{delete, post, put};
use axum::{extract::Path, http::StatusCode, routing::get, Json, Router, Server};
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
                Router::new()
                    .route("/:id", get(get_protagonist))
                    .route("/", post(create_protagonist))
                    .route("/", put(update_protagonist))
                    .route("/:id", delete(delete_protagonist)),
            )
            .nest(
                "/supporter",
                Router::new()
                    .route("/:id", get(get_supporter))
                    .route("/", post(create_supporter))
                    .route("/", put(update_supporter))
                    .route("/:id", delete(delete_supporter)),
            )
            .nest(
                "/protagonist_supporter",
                Router::new()
                    .route("/:id", get(get_protagonist_supporter))
                    .route("/", post(create_protagonist_supporter))
                    .route("/", put(update_protagonist_supporter))
                    .route("/:id", delete(delete_protagonist_supporter)),
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

async fn get_protagonist(
    Path(id): Path<u64>,
    Query(query): Query<GetProtagonistRequest>,
) -> Result<Json<GetProtagonistResponse>, StatusCode> {
    info!("Get protagonist");

    info!("Protagonist id: {}", id);
    info!("Query: {:?}", query);

    Ok(Json(GetProtagonistResponse {
        id: id,
        name: "Alice".to_string(),
    }))
}

async fn create_protagonist(
    Json(body): Json<CreateProtagonistRequest>,
) -> Result<Json<CreateProtagonistResponse>, StatusCode> {
    info!("Create protagonist");

    if body.name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    info!("Protagonist name: {}", body.name);

    Ok(Json(CreateProtagonistResponse {
        status: "The protagonist has been created".to_string(),
    }))
}

async fn update_protagonist(
    Json(body): Json<UpdateProtagonistRequest>,
) -> Result<Json<UpdateProtagonistResponse>, StatusCode> {
    info!("Update protagonist");

    if body.name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    info!("Protagonist name: {}", body.name);

    Ok(Json(UpdateProtagonistResponse {
        status: "The protagonist has been updated".to_string(),
    }))
}

async fn delete_protagonist(
    Path(id): Path<u64>,
) -> Result<Json<DeleteProtagonistResponse>, StatusCode> {
    info!("Delete protagonist");

    info!("Protagonist id: {}", id);

    Ok(Json(DeleteProtagonistResponse {
        status: "The protagonist has been deleted".to_string(),
    }))
}

async fn get_supporter(
    Path(id): Path<u64>,
    Query(query): Query<GetSupporterRequest>,
) -> Result<Json<GetSupporterResponse>, StatusCode> {
    info!("Get supporter");

    info!("Supporter id: {}", id);
    info!("Query: {:?}", query);

    Ok(Json(GetSupporterResponse {
        id: 1,
        name: "Bob".to_string(),
    }))
}

async fn create_supporter(
    Json(body): Json<CreateSupporterRequest>,
) -> Result<Json<CreateSupporterResponse>, StatusCode> {
    info!("Create supporter");

    if body.name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    info!("Supporter name: {}", body.name);

    Ok(Json(CreateSupporterResponse {
        status: "The supporter has been created".to_string(),
    }))
}

async fn update_supporter(
    Json(body): Json<UpdateSupporterRequest>,
) -> Result<Json<UpdateSupporterResponse>, StatusCode> {
    info!("Update supporter");

    if body.name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    info!("Supporter name: {}", body.name);

    Ok(Json(UpdateSupporterResponse {
        status: "The supporter has been updated".to_string(),
    }))
}

async fn delete_supporter(
    Path(id): Path<u64>,
) -> Result<Json<DeleteSupporterResponse>, StatusCode> {
    info!("Delete supporter");

    info!("Supporter id: {}", id);

    Ok(Json(DeleteSupporterResponse {
        status: "The supporter has been deleted".to_string(),
    }))
}

async fn get_protagonist_supporter(
    Path(id): Path<u64>,
    Query(query): Query<GetProtagonistRequest>,
) -> Result<Json<GetProtagonistSupporterResponse>, StatusCode> {
    info!("Get protagonist supporter");

    info!("Protagonist supporter id: {}", id);
    info!("Query: {:?}", query);

    Ok(Json(GetProtagonistSupporterResponse {
        protagonist_id: 1,
        protagonist_name: "Alice".to_string(),
        supporter_id: 1,
        supporter_name: "Bob".to_string(),
    }))
}

async fn create_protagonist_supporter(
    Json(body): Json<CreateProtagonistSupporterRequest>,
) -> Result<Json<CreateProtagonistSupporterResponse>, StatusCode> {
    info!("Create protagonist supporter");

    info!("Protagonist id: {}", body.protagonist_id);
    info!("Supporter id: {}", body.supporter_id);

    Ok(Json(CreateProtagonistSupporterResponse {
        status: "The protagonist supporter has been created".to_string(),
    }))
}

async fn update_protagonist_supporter(
    Json(body): Json<UpdateProtagonistSupporterRequest>,
) -> Result<Json<UpdateProtagonistSupporterResponse>, StatusCode> {
    info!("Update protagonist supporter");

    // check if the protagonist supporter exists or not or int
    if body.id == 0 || body.protagonist_id == 0 || body.supporter_id == 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    info!("Protagonist id: {}", body.protagonist_id);
    info!("Supporter id: {}", body.supporter_id);
    info!("Protagonist supporter id: {}", body.id);

    Ok(Json(UpdateProtagonistSupporterResponse {
        status: "The protagonist supporter has been updated".to_string(),
    }))
}

async fn delete_protagonist_supporter(
    Path(id): Path<u64>,
) -> Result<Json<DeleteProtagonistSupporterResponse>, StatusCode> {
    info!("Delete protagonist supporter");

    info!("Protagonist supporter id: {}", id);

    Ok(Json(DeleteProtagonistSupporterResponse {
        status: "The protagonist supporter has been deleted".to_string(),
    }))
}
