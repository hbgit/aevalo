mod modules;
mod models;
mod db;
mod error;
mod handlers;
mod middleware;

use axum::{
    routing::{get, post, patch},
    middleware as axum_middleware,
    Router,
    extract::State,
    http::HeaderValue,
};
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tracing::info;

use middleware::security_headers::*;
use middleware::auth::{require_auth, optional_auth};

/// Simple health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .init();
    
    info!("🚀 Starting Aevalo Backend...");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database connection
    let pool = db::init_pool().await?;
    info!("✅ Database connected");

    // Protected routes with auth middleware
    let protected_routes = Router::new()
        // Evaluation endpoints
        .route("/evaluations", 
            get(handlers::evaluations::list_evaluations)
                .post(handlers::evaluations::create_evaluation))
        .route("/evaluations/{id}", 
            get(handlers::evaluations::get_evaluation)
                .patch(handlers::evaluations::update_evaluation))
        .route("/evaluations/{id}/publish", 
            post(handlers::evaluations::publish_evaluation))
        .route("/evaluations/{id}/close", 
            post(handlers::evaluations::close_evaluation))

        // AI Generation
        .route("/evaluations/generate", 
            post(handlers::ai_generation::generate_items_ai))
        .route("/evaluations/validate", 
            post(handlers::ai_generation::validate_items))

        // Responses
        .route("/evaluations/{id}/responses", 
            get(handlers::responses::get_responses))
        .route("/evaluations/{id}/stats", 
            get(handlers::responses::get_response_stats))

        // Analytics
        .route("/evaluations/{id}/process", 
            post(handlers::analytics::process_evaluation))
        .route("/evaluations/{id}/results", 
            get(handlers::analytics::get_results))

        // User endpoints
        .route("/user/profile", 
            get(handlers::auth::get_user_profile))
        .route("/user/preferences", 
            get(handlers::auth::get_user_preferences)
                .patch(handlers::auth::update_user_preferences))

        .layer(axum_middleware::from_fn_with_state(
            pool.clone(),
            require_auth,
        ));

    // Build main router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))

        // Authentication endpoints
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/auth/refresh", post(handlers::auth::refresh_token))

        // Public endpoints
        .route("/responses", post(handlers::responses::submit_responses))
        .route("/public/eval/{uuid}", 
            get(handlers::public::get_public_evaluation))
        .route("/public/eval/{uuid}/stats", 
            get(handlers::public::get_public_stats))

        // Protected endpoints
        .merge(protected_routes)

        // Security headers
        .layer(csp_header_layer())
        .layer(x_content_type_options_layer())
        .layer(x_frame_options_layer())
        .layer(x_xss_protection_layer())
        .layer(hsts_header_layer())
        .layer(referrer_policy_layer())

        // CORS configuration
        .layer(
            CorsLayer::very_permissive()
                .allow_credentials(false)
        )

        .with_state(pool);

    // Start server
    let addr = std::env::var("SERVER_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", addr, port)).await?;
    info!("📡 Server listening on http://{}:{}", addr, port);
    
    axum::serve(listener, app).await?;

    Ok(())
}

