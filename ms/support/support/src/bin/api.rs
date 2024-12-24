use dotenv::dotenv;
use std::env;
use tracing::error;
use tracing_subscriber::{
    layer::SubscriberExt, 
    util::SubscriberInitExt
};
use lib::{
    driver::database::new_database,
    router::router::AppRouter
};

const SUPPORT_PG_URL: &'static str = "SUPPORT_PG_URL";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();
    if env::var(SUPPORT_PG_URL).is_err() {
        error!("{} is not set", SUPPORT_PG_URL);
        return;
    }

    let url = env::var(SUPPORT_PG_URL).unwrap();
    if let Err(e) = new_database(&url).await {
        error!("Error: {:?}", e);
        return;
    }

    let router = AppRouter::new();
    if let Err(e) = router.serve().await {
        error!("Error: {:?}", e);
    }
}
