use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{Coordinate, Grid, Robot, RobotState};
use crate::pathfinding::a_star;
use crate::state::SharedState;

#[derive(Deserialize)]
pub struct DispatchRequest {
    pub robot_id: String,
    pub destination: Coordinate,
}

#[derive(Serialize)]
pub struct DispatchResponse {
    pub path: Vec<Coordinate>,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub grid: Grid,
    pub robots: std::collections::HashMap<String, Robot>,
}

#[derive(Deserialize)]
pub struct ObstacleRequest {
    pub x: usize,
    pub y: usize,
}

#[autometrics::autometrics]
pub async fn dispatch_robot(
    State(state): State<SharedState>,
    Json(payload): Json<DispatchRequest>,
) -> Result<Json<DispatchResponse>, (StatusCode, String)> {
    let mut state_write = state.write().await;
    
    // Check if robot exists, or create a new one for simulation at a random/default position
    // For simplicity, if robot doesn't exist, we start it at (0,0) or some default
    let current_position = if let Some(robot) = state_write.robots.get(&payload.robot_id) {
        robot.current_position
    } else {
        // Create new robot at 0,0
        let coord = Coordinate { x: 0, y: 0 };
        state_write.robots.insert(payload.robot_id.clone(), Robot {
            id: payload.robot_id.clone(),
            current_position: coord,
            state: RobotState::Idle,
        });
        coord
    };

    let occupied = state_write.get_occupied_coordinates(Some(&payload.robot_id));
    
    match a_star(&state_write.grid, current_position, payload.destination, &occupied) {
        Some(path) => {
            tracing::info!("Found path for robot {}: {:?}", payload.robot_id, path);
            
            // Update robot state
            if let Some(robot) = state_write.robots.get_mut(&payload.robot_id) {
                robot.state = RobotState::Moving;
                // Ideally, a background task would move the robot step-by-step.
                // For this simulation, we'll just snap it to the destination or just store the reserved path.
                robot.current_position = payload.destination; // Simulating immediate teleportation for now
            }
            
            // Reserve path (if we were stepping over time, we'd remove them as the robot moves)
            state_write.reserved_paths.insert(payload.robot_id.clone(), path.clone());
            
            Ok(Json(DispatchResponse { path }))
        }
        None => {
            tracing::warn!("No path found for robot {}", payload.robot_id);
            if let Some(robot) = state_write.robots.get_mut(&payload.robot_id) {
                robot.state = RobotState::Error;
            }
            Err((StatusCode::NOT_FOUND, "No path found".to_string()))
        }
    }
}

#[autometrics::autometrics]
pub async fn get_status(State(state): State<SharedState>) -> impl IntoResponse {
    let state_read = state.read().await;
    Json(StatusResponse {
        grid: state_read.grid.clone(),
        robots: state_read.robots.clone(),
    })
}

#[autometrics::autometrics]
pub async fn add_obstacle(
    State(state): State<SharedState>,
    Json(payload): Json<ObstacleRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut state_write = state.write().await;
    let coord = Coordinate { x: payload.x, y: payload.y };
    
    if payload.x >= state_write.grid.width || payload.y >= state_write.grid.height {
        return Err((StatusCode::BAD_REQUEST, "Obstacle out of bounds".to_string()));
    }
    
    state_write.grid.add_obstacle(payload.x, payload.y);
    tracing::info!("Added obstacle at {:?}", coord);
    Ok(StatusCode::CREATED)
}

pub async fn metrics_endpoint() -> impl IntoResponse {
    let prometheus = autometrics::prometheus_exporter::encode_to_string().unwrap_or_default();
    (StatusCode::OK, prometheus)
}
