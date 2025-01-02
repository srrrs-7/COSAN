use dotenv::dotenv;
use lib::{
    domain::service::SupportService,
    driver::{database::new_database, repository::SupportRepository},
    router::router::AppRouter,
    util::slog::new_logger,
};
use std::env;
use tracing::{error, info, span, Level};

const MODE: &'static str = "MODE";
const SUPPORT_PG_URL: &'static str = "SUPPORT_PG_URL";
const SECRET_KEY: &'static str = "SECRET_KEY";

#[derive(Debug)]
struct Env {
    mode: String,
    support_pg_url: String,
    secret_key: String,
}

async fn init_env() -> Env {
    dotenv().ok();
    if env::var(MODE).is_err() | env::var(SUPPORT_PG_URL).is_err() | env::var(SECRET_KEY).is_err() {
        error!("{} is not set", SUPPORT_PG_URL);
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
        support_pg_url: env::var(SUPPORT_PG_URL).unwrap(),
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

    let support_service = SupportService::new(SupportRepository::new(
        new_database(&env.support_pg_url).await.unwrap(),
    ));

    let router = AppRouter::new(support_service, env.secret_key);
    router.serve().await.unwrap();
}
