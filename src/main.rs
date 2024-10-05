use std::env;
use axum::{
    Router,
    routing::get
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| format!("{}=debug", env::var("APPLICATION_PROFILE").unwrap()).into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let app = Router::new()
    .route("/healthz", get(health_check));

    let addr = format!("{}:{}", env::var("HOST").unwrap(), env::var("PORT").unwrap());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::debug!("server listening on: {}", addr);
    axum::serve(listener, app).await.unwrap();
}


async fn health_check() -> &'static str {
    "Health check OK"
}