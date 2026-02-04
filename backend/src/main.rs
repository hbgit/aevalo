mod modules;
mod graphql;
mod models;
mod db;
mod error;
mod handlers;

use axum::{
    routing::{get, post, patch},
    Router,
    extract::State,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing::info;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Aevalo Backend...");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/aevalo".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    info!("✅ Database connected");

    // Build router with all handlers
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Authentication
        .route("/auth/login", post(handlers::auth::login))
        
        // Evaluation endpoints
        .route("/evaluations", get(handlers::evaluations::list_evaluations)
            .post(handlers::evaluations::create_evaluation))
        .route("/evaluations/:id", get(handlers::evaluations::get_evaluation)
            .patch(handlers::evaluations::update_evaluation))
        .route("/evaluations/:id/publish", post(handlers::evaluations::publish_evaluation))
        .route("/evaluations/:id/close", post(handlers::evaluations::close_evaluation))
        
        // AI Generation
        .route("/evaluations/generate", post(handlers::ai_generation::generate_items_ai))
        .route("/evaluations/validate", post(handlers::ai_generation::validate_items))
        
        // Responses
        .route("/responses", post(handlers::responses::submit_responses))
        .route("/evaluations/:id/responses", get(handlers::responses::get_responses))
        .route("/evaluations/:id/stats", get(handlers::responses::get_response_stats))
        
        // Analytics
        .route("/evaluations/:id/process", post(handlers::analytics::process_evaluation))
        .route("/evaluations/:id/results", get(handlers::analytics::get_results))
        
        // Public endpoints (unauthenticated)
        .route("/public/eval/:uuid", get(handlers::public::get_public_evaluation))
        .route("/public/eval/:uuid/stats", get(handlers::public::get_public_stats))
        
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(pool));

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("📡 Server listening on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "✅ Backend is healthy!"
}
