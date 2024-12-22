use axum::{
    routing::get,
    Router,
};

pub async fn new_router() {
    let router = Router::new();

    router.route("/", get(health_check));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn health_check() -> &'static str {
    "OK"
}
