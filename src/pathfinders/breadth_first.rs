use crate::pathfinders::{Grid, PathFinder, Pos, Tile, Vec2d};

pub struct BreadthFirst {}

impl PathFinder for BreadthFirst {
    fn find_path(grid: &Grid) -> Option<Vec<Pos>> {
        let mut queue = vec![];
        let mut backtrace: Vec2d<Option<Pos>> =
            Vec2d::new(grid.rows() as usize, grid.columns() as usize, None);

        queue.push(grid.start());

        while let Some(pos) = queue.pop() {
            if pos == grid.end() {
                break;
            }

            const DIRECTIONS: [Pos; 4] = [
                Pos { x: 0, y: -1 },
                Pos { x: 1, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: -1, y: 0 },
            ];

            let neighbors = DIRECTIONS
                .iter()
                .map(|dir| pos + *dir)
                .filter(|&pos| grid.tile_opt(pos) == Some(Tile::None));

            for neighbor in neighbors {
                // if backtrace[neighbor] doesn't exist
                if let Some(None) = backtrace.get(neighbor) {
                    backtrace.set(neighbor, Some(pos));
                    queue.push(neighbor);
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
