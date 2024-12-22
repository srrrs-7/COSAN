mod router {
    pub mod router;
}

use crate::router::router::new_router;

#[tokio::main]
async fn main() {
    new_router().await;
}