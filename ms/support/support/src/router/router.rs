use axum::{
    routing::get,
    Router,
    Server,
    http::StatusCode,
};
use std::net::SocketAddr;

async fn health_check() -> Result<&'static str, StatusCode> {
    Ok("OK")
}

pub async fn new_router() {
    let router = Router::new().route("/", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}