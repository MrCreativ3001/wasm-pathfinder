use crate::pathfinders::best_first::{BestFirst, PosPrioritizer};
use crate::pathfinders::{Grid, Pos, Vec2d};
use std::collections::VecDeque;

pub type Dijkstra = BestFirst<DijkstraPrioritizer>;

pub struct DijkstraPrioritizer {
    grid_start: Pos,
    cached_distances_from_start: Vec2d<Option<f32>>,
}

impl DijkstraPrioritizer {
    fn distance_from_start(&mut self, backtrace: &Vec2d<Option<Pos>>, pos: Pos) -> Option<f32> {
        let mut distance = 0.0;
        let mut current_pos = pos;

        while let Some(Some(parent_pos)) = backtrace.get(current_pos) {
            if let Some(Some(parent_pos_dist)) = self.cached_distances_from_start.get(*parent_pos) {
                distance += parent_pos_dist;
                self.cached_distances_from_start.set(pos, Some(distance));
                return Some(distance);
            }

            distance += 1.0;
            current_pos = *parent_pos;
            if *parent_pos == self.grid_start {
                self.cached_distances_from_start.set(pos, Some(distance));
                return Some(distance);
            }
        }

        None
    }
}

impl PosPrioritizer for DijkstraPrioritizer {
    fn new_prioritizer(grid: &Grid) -> Self {
        Self {
            grid_start: grid.start(),
            cached_distances_from_start: Vec2d::new(
                grid.rows() as usize,
                grid.columns() as usize,
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
            let pos_distance = self
                .distance_from_start(backtrace, *pos)
                .unwrap_or(f32::MAX);
            if pos_distance < distance {
                distance = pos_distance;
                prioritized_pos = i;
            }
        }

        prioritized_pos
    }
}
