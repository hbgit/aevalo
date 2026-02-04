mod modules;
mod graphql;
mod models;
mod db;
mod error;

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Aevalo Backend...");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database connection
    // let db = db::init().await?;
    // let db = Arc::new(db);

    // Build GraphQL schema
    // let schema = graphql::build_schema(db.clone()).await;

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive());

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("📡 Server listening on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "✅ Backend is healthy!"
}
