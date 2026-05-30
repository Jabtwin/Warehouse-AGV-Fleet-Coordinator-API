use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub obstacles: std::collections::HashSet<Coordinate>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            obstacles: std::collections::HashSet::new(),
        }
    }

    pub fn add_obstacle(&mut self, x: usize, y: usize) {
        self.obstacles.insert(Coordinate { x, y });
    }

    pub fn is_valid_and_unblocked(&self, coord: &Coordinate) -> bool {
        coord.x < self.width && coord.y < self.height && !self.obstacles.contains(coord)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RobotState {
    Idle,
    Moving,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robot {
    pub id: String,
    pub current_position: Coordinate,
    pub state: RobotState,
}
