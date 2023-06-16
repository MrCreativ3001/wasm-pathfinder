use crate::pathfinders::best_first::{BestFirst, PosPrioritizer};
use crate::pathfinders::{Grid, Pos, Vec2d};
use std::collections::VecDeque;

pub type BreadthFirst = BestFirst<BreadthFirstPrioritizer>;

pub struct BreadthFirstPrioritizer;

impl PosPrioritizer for BreadthFirstPrioritizer {
    fn new_prioritizer(_grid: &Grid) -> Self {
        Self
    }

    fn find_prioritized_pos(
        &mut self,
        _queue: &VecDeque<Pos>,
        _backtrace: &Vec2d<Option<Pos>>,
    ) -> usize {
        0
    }
}
