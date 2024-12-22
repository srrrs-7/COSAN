use tracing::error;
use dotenv::dotenv;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod router {
    pub mod router;
    pub mod response;
}
use crate::router::router::new_router;
mod driver {
    pub mod driver;
}
use crate::driver::driver::new_database;

const SUPPORT_PG_URL: &'static str = "SUPPORT_PG_URL";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();
    if let Err(e) = new_database(&env::var(SUPPORT_PG_URL).expect("DATABASE_URL must be set")).await {
        error!("Error: {:?}", e);
    }

    if let Err(e) = new_router().await {
        error!("Error: {:?}", e);
    }
}