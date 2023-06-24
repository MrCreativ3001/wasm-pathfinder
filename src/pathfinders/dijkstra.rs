use crate::pathfinders::best_first::{BestFirst, PosPrioritizer};
use crate::pathfinders::distance::distance_to_start;
use crate::pathfinders::{Grid, Pos, Vec2d};
use std::collections::VecDeque;

pub type Dijkstra = BestFirst<DijkstraPrioritizer>;

pub struct DijkstraPrioritizer {
    grid_start: Pos,
    cached_distances_from_start: Vec2d<Option<f32>>,
}

impl DijkstraPrioritizer {}

impl PosPrioritizer for DijkstraPrioritizer {
    fn new_prioritizer(grid: &Grid) -> Self {
        Self {
            grid_start: grid.start(),
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
            let pos_distance = distance_to_start(
                &mut self.cached_distances_from_start,
                self.grid_start,
                backtrace,
                *pos,
            )
            .unwrap_or(f32::MAX);
            if pos_distance < distance {
                distance = pos_distance;
                prioritized_pos = i;
            }
        }

        prioritized_pos
    }
}
