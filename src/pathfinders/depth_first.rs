use crate::pathfinders::best_first::{BestFirst, PosPrioritizer};
use crate::pathfinders::{Grid, Pos, Vec2d};
use std::collections::VecDeque;

pub type DepthFirst = BestFirst<DepthFirstPrioritizer>;

pub struct DepthFirstPrioritizer;

impl PosPrioritizer for DepthFirstPrioritizer {
    fn new_prioritizer(_grid: &Grid) -> Self {
        Self
    }

    fn find_prioritized_pos(
        &mut self,
        queue: &VecDeque<Pos>,
        _backtrace: &Vec2d<Option<Pos>>,
    ) -> usize {
        queue.len() - 1
    }
}
