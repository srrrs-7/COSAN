use dotenv::dotenv;
use lib::{driver::database::new_database, router::router::AppRouter, util::slog::new_logger};
use std::env;
use tracing::{error, info, span, Level};

const SUPPORT_PG_URL: &'static str = "SUPPORT_PG_URL";

#[derive(Debug)]
struct Env {
    support_pg_url: String,
}

async fn init_env() -> Env {
    dotenv().ok();
    if env::var(SUPPORT_PG_URL).is_err() {
        error!("{} is not set", SUPPORT_PG_URL);
        panic!();
    }

    Env {
        support_pg_url: env::var(SUPPORT_PG_URL).unwrap(),
    }
}

#[tokio::main]
async fn main() {
    new_logger().await;
    let _span = span!(Level::INFO, "main");

    let env = init_env().await;
    info!("Env: {:?}", env);

    if let Err(e) = new_database(&env.support_pg_url).await {
        error!("Error: {:?}", e);
        panic!();
    }

    let router = AppRouter::new();
    if let Err(e) = router.serve().await {
        error!("Error: {:?}", e);
        panic!();
    }
}
