mod router {
    pub mod router;
    pub mod response;
}
use crate::router::router::new_router;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .init();

    if let Err(e) = new_router().await {
        error!("Error: {:?}", e);
    }
}