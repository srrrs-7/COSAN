use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use sqlx::PgPool;

pub async fn new_database(url: &str) -> Result<PgPool, Error> {
    Ok(PgPoolOptions::new().max_connections(5).connect(url).await?)
}
