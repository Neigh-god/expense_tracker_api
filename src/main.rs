use std::net::SocketAddr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "expense_tracker_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env();
    let pool = db::create_pool(&config).await?;
    
    sqlx::query("SELECT 1").fetch_one(&pool).await?;
    tracing::info!("Database connected successfully");

    let app = routes::api::create_api_router(pool, config.clone());
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    
    tracing::info!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let _ = axum::serve(listener, app).await;

    Ok(())
}
