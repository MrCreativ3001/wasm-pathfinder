use crate::pathfinders::PathFindAlgorithmStepResult::{InProgress, NotFound};
use crate::pathfinders::{
    Grid, PathFindAlgorithm, PathFindAlgorithmConstructor, PathFindAlgorithmStepResult, Pos, Tile,
    Vec2d,
};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct BreadthFirst {
    grid: Grid,
    queue: VecDeque<Pos>,
    backtrace: Vec2d<Option<Pos>>,
}

impl BreadthFirst {
    fn init(&mut self) {
        self.queue.push_front(self.grid.start());
    }
}

impl PathFindAlgorithmConstructor for BreadthFirst {
    fn make_state(grid: Grid) -> Self {
        let mut state = Self {
            backtrace: Vec2d::new(grid.rows() as usize, grid.columns() as usize, None),
            queue: VecDeque::new(),
            grid,
        };
        state.init();
        state
    }
}
impl PathFindAlgorithm for BreadthFirst {
    fn next_step(&mut self) -> Result<Vec<Pos>, PathFindAlgorithmStepResult> {
        const DIRECTIONS: [Pos; 4] = [Pos::UP, Pos::DOWN, Pos::LEFT, Pos::RIGHT];

        // if the queue is empty, no more tiles to search exist
        let pos = self.queue.pop_back().ok_or(NotFound)?;

        // if the tile is the end, try to find the path
        if pos == self.grid.end() {
            // backtrace
            let mut path = Vec::new();
            let mut pos = pos;
            while pos != self.grid.start() {
                path.push(pos);
                // If no backtrace, no path was found
                pos = self.backtrace.get(pos).ok_or(NotFound)?.ok_or(NotFound)?;
            }
            path.push(pos);
            path.reverse();
            return Ok(path);
        }

        let neighbors = DIRECTIONS
            .iter()
            .map(|dir| pos + *dir)
            .filter(|pos| !self.visited(*pos))
            .filter(|pos| matches!(self.grid.tile_opt(*pos), Some(Tile::None)))
            .collect::<Vec<_>>();

        for neighbor in neighbors {
            if matches!(self.backtrace.get(neighbor), Some(Some(_))) {
                continue;
            }
            self.queue.push_front(neighbor);
            self.backtrace.set(neighbor, Some(pos));
        }

        Err(InProgress)
    }

    fn visited(&self, pos: Pos) -> bool {
        match self.backtrace.get(pos) {
            Some(Some(_)) => true,
            _ => false,
        }
    }

    fn in_queue(&self, pos: Pos) -> bool {
        self.queue.contains(&pos)
    }
}
