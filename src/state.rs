use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{Coordinate, Grid, Robot};

pub struct WarehouseState {
    pub grid: Grid,
    pub robots: HashMap<String, Robot>,
    pub reserved_paths: HashMap<String, Vec<Coordinate>>, // Robot ID to planned path
}

pub type SharedState = Arc<RwLock<WarehouseState>>;

impl WarehouseState {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            robots: HashMap::new(),
            reserved_paths: HashMap::new(),
        }
    }

    pub fn get_occupied_coordinates(
        &self,
        excluding_robot_id: Option<&str>,
    ) -> HashSet<Coordinate> {
        let mut occupied = HashSet::new();

        for robot in self.robots.values() {
            if Some(robot.id.as_str()) != excluding_robot_id {
                occupied.insert(robot.current_position);
            }
        }

        for (id, path) in &self.reserved_paths {
            if Some(id.as_str()) != excluding_robot_id {
                for coord in path {
                    occupied.insert(*coord);
                }
            }
        }

        occupied
    }
}
