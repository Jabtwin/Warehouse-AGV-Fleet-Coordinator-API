use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::models::{Coordinate, Grid};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    coord: Coordinate,
    cost: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare coordinates - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.coord.x.cmp(&other.coord.x))
            .then_with(|| self.coord.y.cmp(&other.coord.y))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(a: &Coordinate, b: &Coordinate) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

#[autometrics::autometrics]
pub fn a_star(
    grid: &Grid,
    start: Coordinate,
    goal: Coordinate,
    occupied_coords: &std::collections::HashSet<Coordinate>,
) -> Option<Vec<Coordinate>> {
    if !grid.is_valid_and_unblocked(&start) || !grid.is_valid_and_unblocked(&goal) {
        return None;
    }
    
    // Quick check if goal is blocked by other robots (unless it's the start node)
    if start != goal && occupied_coords.contains(&goal) {
        return None;
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Coordinate, Coordinate> = HashMap::new();
    let mut g_score: HashMap<Coordinate, usize> = HashMap::new();

    g_score.insert(start, 0);
    open_set.push(Node {
        coord: start,
        cost: manhattan_distance(&start, &goal),
    });

    while let Some(Node { coord: current, .. }) = open_set.pop() {
        if current == goal {
            let mut path = vec![current];
            let mut curr = current;
            while let Some(&prev) = came_from.get(&curr) {
                path.push(prev);
                curr = prev;
            }
            path.reverse();
            return Some(path);
        }

        let current_g_score = *g_score.get(&current).unwrap_or(&usize::MAX);

        let neighbors = [
            Coordinate { x: current.x.saturating_add(1), y: current.y },
            Coordinate { x: current.x.saturating_sub(1), y: current.y },
            Coordinate { x: current.x, y: current.y.saturating_add(1) },
            Coordinate { x: current.x, y: current.y.saturating_sub(1) },
        ];

        for next in neighbors {
            // Avoid adding start node back, or invalid moves
            if next == current || !grid.is_valid_and_unblocked(&next) {
                continue;
            }
            
            // Avoid collisions with other robots
            if next != start && occupied_coords.contains(&next) {
                continue;
            }

            let tentative_g_score = current_g_score + 1;
            let next_g_score = *g_score.get(&next).unwrap_or(&usize::MAX);

            if tentative_g_score < next_g_score {
                came_from.insert(next, current);
                g_score.insert(next, tentative_g_score);
                let f_score = tentative_g_score + manhattan_distance(&next, &goal);
                open_set.push(Node {
                    coord: next,
                    cost: f_score,
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_line() {
        let grid = Grid::new(10, 10);
        let empty_occupied = std::collections::HashSet::new();
        let path = a_star(
            &grid,
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 3, y: 0 },
            &empty_occupied,
        )
        .unwrap();
        
        assert_eq!(
            path,
            vec![
                Coordinate { x: 0, y: 0 },
                Coordinate { x: 1, y: 0 },
                Coordinate { x: 2, y: 0 },
                Coordinate { x: 3, y: 0 },
            ]
        );
    }

    #[test]
    fn test_with_obstacle() {
        let mut grid = Grid::new(5, 5);
        grid.add_obstacle(1, 0);
        grid.add_obstacle(1, 1);
        let empty_occupied = std::collections::HashSet::new();
        
        // Start (0,0) -> Goal (2,0)
        // Must go down to (0,2), right to (2,2), then up to (2,0) or similar around the obstacles
        let path = a_star(
            &grid,
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 2, y: 0 },
            &empty_occupied,
        );
        
        assert!(path.is_some());
        let p = path.unwrap();
        assert_eq!(*p.first().unwrap(), Coordinate { x: 0, y: 0 });
        assert_eq!(*p.last().unwrap(), Coordinate { x: 2, y: 0 });
        assert!(!p.contains(&Coordinate { x: 1, y: 0 }));
        assert!(!p.contains(&Coordinate { x: 1, y: 1 }));
    }

    #[test]
    fn test_no_path() {
        let mut grid = Grid::new(3, 3);
        // Wall off (2,2)
        grid.add_obstacle(1, 2);
        grid.add_obstacle(2, 1);
        grid.add_obstacle(1, 1);
        
        let empty_occupied = std::collections::HashSet::new();
        
        let path = a_star(
            &grid,
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 2, y: 2 },
            &empty_occupied,
        );
        
        assert!(path.is_none());
    }
}
