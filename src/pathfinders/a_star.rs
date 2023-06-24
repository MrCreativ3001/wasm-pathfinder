use crate::pathfinders::best_first::{BestFirst, PosPrioritizer};
use crate::pathfinders::distance::{distance_to_start, guess_distance};
use crate::pathfinders::{Grid, Pos, Vec2d};
use std::collections::VecDeque;

pub type AStar = BestFirst<AStarPrioritizer>;

pub struct AStarPrioritizer {
    grid_start: Pos,
    grid_end: Pos,
    cached_distances_from_start: Vec2d<Option<f32>>,
}

impl PosPrioritizer for AStarPrioritizer {
    fn new_prioritizer(grid: &Grid) -> Self {
        Self {
            grid_start: grid.start(),
            grid_end: grid.end(),
            cached_distances_from_start: Vec2d::new(
                grid.height() as usize,
                grid.width() as usize,
                None,
            ),
        }
    }

    fn find_prioritized_pos(
        &mut self,
        queue: &VecDeque<Pos>,
        backtrace: &Vec2d<Option<Pos>>,
    ) -> usize {
        let mut distance = f32::MAX;
        let mut prioritized_pos = 0;

        for (i, pos) in queue.iter().enumerate() {
            let pos_distance_start = distance_to_start(
                &mut self.cached_distances_from_start,
                self.grid_start,
                backtrace,
                *pos,
            )
            .unwrap_or(f32::MAX);
            // we can only guess the distance to the end as we don't know the path yet
            let pos_distance_end = guess_distance(*pos, self.grid_end);

            let pos_distance = pos_distance_start + pos_distance_end;

            if pos_distance < distance {
                distance = pos_distance;
                prioritized_pos = i;
            }
        }

        prioritized_pos
    }
}
