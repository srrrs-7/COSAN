use tracing::error;
use dotenv::dotenv;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use support::router::router::AppRouter;

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

    if let Err(e) = AppRouter::new().serve().await {
        error!("Error: {:?}", e);
    }
}