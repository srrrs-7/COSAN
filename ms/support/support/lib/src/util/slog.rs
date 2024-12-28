use slog::{o, Drain, Logger};
use slog_async;
use slog_json;
use tracing::Level;
use tracing_subscriber;

pub async fn new_logger() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(Level::INFO)
        .init();

    let drain = slog_json::Json::default(std::io::stdout()).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!());

    let span = tracing::span!(Level::INFO, "my_span");
    let _enter = span.enter();

    slog::info!(log, "Logger initialized");
}
