use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;

use agv_coordinator::api;
use agv_coordinator::state::WarehouseState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("agv_coordinator=debug".parse().unwrap()))
        .init();

    // Initialize autometrics prometheus exporter
    autometrics::prometheus_exporter::init();

    // Initialize Global State (Grid 100x100 for default as requested)
    let state = Arc::new(RwLock::new(WarehouseState::new(100, 100)));

    // Setup Router
    let app = Router::new()
        .route("/api/fleet/dispatch", post(api::dispatch_robot))
        .route("/api/fleet/status", get(api::get_status))
        .route("/api/grid/obstacle", post(api::add_obstacle))
        .route("/metrics", get(api::metrics_endpoint))
        .with_state(state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}
