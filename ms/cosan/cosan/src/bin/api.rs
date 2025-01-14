use dotenv::dotenv;
use lib::{
    domain::service::CosanService,
    driver::{database::new_database, repository::CosanRepository},
    router::router::AppRouter,
    util::slog::new_logger,
};
use std::env;
use tracing::{error, info, span, Level};

const MODE: &'static str = "MODE";
const COSAN_PG_URL: &'static str = "COSAN_PG_URL";
const SECRET_KEY: &'static str = "SECRET_KEY";

#[derive(Debug)]
struct Env {
    mode: String,
    cosan_pg_url: String,
    secret_key: String,
}

async fn init_env() -> Env {
    dotenv().ok();
    if env::var(MODE).is_err() | env::var(COSAN_PG_URL).is_err() | env::var(SECRET_KEY).is_err() {
        error!("{} is not set", COSAN_PG_URL);
        panic!();
    }

    let mode = env::var(MODE).unwrap_or(format!("debug"));
    match mode {
        ref mode if mode == "debug" || mode == "release" => {}
        _ => {
            error!("{} is not valid", MODE);
            panic!();
        }
    }

    Env {
        mode,
        cosan_pg_url: env::var(COSAN_PG_URL).unwrap(),
        secret_key: env::var(SECRET_KEY).unwrap(),
    }
}

#[tokio::main]
async fn main() {
    new_logger().await;
    let _span = span!(Level::INFO, "main");

    let env = init_env().await;
    info!("Env: {:?}", env);

    if env.mode == "debug" {
        info!("Running in debug mode");
    } else {
        info!("Running in release mode");
    }

    let cosan_service = CosanService::new(CosanRepository::new(
        new_database(&env.cosan_pg_url).await.unwrap(),
    ));

    let router = AppRouter::new(cosan_service, env.secret_key);
    router.serve().await.unwrap();
}
