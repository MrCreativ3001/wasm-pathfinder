use crate::pathfinders::{Grid, PathFinder, Pos, Tile, Vec2d};
use std::collections::VecDeque;

pub struct BreadthFirst {}

impl PathFinder for BreadthFirst {
    fn find_path(grid: &Grid) -> Option<Vec<Pos>> {
        let mut queue = VecDeque::new();
        let mut backtrace: Vec2d<Option<Pos>> =
            Vec2d::new(grid.rows() as usize, grid.columns() as usize, None);

        queue.push_front(grid.start());

        while let Some(pos) = queue.pop_back() {
            if pos == grid.end() {
                break;
            }

            const DIRECTIONS: [Pos; 4] = [Pos::UP, Pos::DOWN, Pos::LEFT, Pos::RIGHT];

            let neighbors = DIRECTIONS
                .iter()
                .map(|dir| pos + *dir)
                .filter(|&pos| grid.tile_opt(pos) == Some(Tile::None));

            for neighbor in neighbors {
                // if backtrace[neighbor] doesn't exist
                if let Some(None) = backtrace.get(neighbor) {
                    backtrace.set(neighbor, Some(pos));
                    queue.push_front(neighbor);
                }
            }
        }

        // backtrace
        let mut path = vec![];

        let mut pos = grid.end();
        while pos != grid.start() {
            path.push(pos);
            pos = match backtrace.get(pos) {
                Some(Some(pos)) => *pos,
                _ => return None,
            }
        }

        path.reverse();

        Some(path)
    }
}
