use crate::pathfinders::PathFindAlgorithmStepResult::{InProgress, NotFound};
use crate::pathfinders::{
    Grid, PathFindAlgorithm, PathFindAlgorithmConstructor, PathFindAlgorithmStepResult, Pos, Tile,
    Vec2d,
};
use std::collections::VecDeque;

pub trait PosPrioritizer {
    fn new_prioritizer(grid: &Grid) -> Self;
    /// Returns the index of the prioritized position in the queue.
    /// The queue is sorted from first(i = 0) added to last(i = queue.len() - 1) added.
    fn find_prioritized_pos(
        &mut self,
        queue: &VecDeque<Pos>,
        backtrace: &Vec2d<Option<Pos>>,
    ) -> usize;
}

#[derive(Clone, Debug)]
pub struct BestFirst<P: PosPrioritizer> {
    grid: Grid,
    queue: VecDeque<Pos>,
    backtrace: Vec2d<Option<Pos>>,
    prioritizer: P,
}

impl<P> BestFirst<P>
where
    P: PosPrioritizer,
{
    fn init(&mut self) {
        self.queue.push_front(self.grid.start());
    }
}

impl<P> PathFindAlgorithmConstructor for BestFirst<P>
where
    P: PosPrioritizer,
{
    fn make_state(grid: Grid) -> Self {
        let mut state = Self {
            backtrace: Vec2d::new(grid.rows() as usize, grid.columns() as usize, None),
            queue: VecDeque::new(),
            prioritizer: P::new_prioritizer(&grid),
            grid,
        };
        state.init();
        state
    }
}
impl<P> PathFindAlgorithm for BestFirst<P>
where
    P: PosPrioritizer,
{
    fn next_step(&mut self) -> Result<Vec<Pos>, PathFindAlgorithmStepResult> {
        const DIRECTIONS: [Pos; 4] = [Pos::UP, Pos::DOWN, Pos::LEFT, Pos::RIGHT];

        // if the queue is empty, no more tiles to search exist
        if self.queue.is_empty() {
            return Err(NotFound);
        }
        let prioritized_pos_i = self
            .prioritizer
            .find_prioritized_pos(&self.queue, &self.backtrace);
        let pos = self
            .queue
            .remove(prioritized_pos_i)
            .expect("NodePrioritizer returned invalid index!");

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
            self.queue.push_back(neighbor);
            self.backtrace.set(neighbor, Some(pos));
        }

        Err(InProgress)
    }

    fn visited(&self, pos: Pos) -> bool {
        matches!(self.backtrace.get(pos), Some(Some(_)))
    }

    fn in_queue(&self, pos: Pos) -> bool {
        self.queue.contains(&pos)
    }
}
