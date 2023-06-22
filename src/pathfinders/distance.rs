use crate::pathfinders::{Pos, Vec2d};

pub fn distance_to_start(
    cached_distances_to_start: &mut Vec2d<Option<f32>>,
    grid_start: Pos,
    backtrace: &Vec2d<Option<Pos>>,
    pos: Pos,
) -> Option<f32> {
    let mut distance = 0.0;
    let mut current_pos = pos;

    while let Some(Some(parent_pos)) = backtrace.get(current_pos) {
        if let Some(Some(parent_pos_dist)) = cached_distances_to_start.get(*parent_pos) {
            distance += parent_pos_dist;
            cached_distances_to_start.set(pos, Some(distance));
            return Some(distance);
        }

        distance += 1.0;
        current_pos = *parent_pos;
        if *parent_pos == grid_start {
            cached_distances_to_start.set(pos, Some(distance));
            return Some(distance);
        }
    }

    None
}

pub fn guess_distance(pos1: Pos, pos2: Pos) -> f32 {
    let x_diff = (pos2.x - pos1.x).abs() as f32;
    let y_diff = (pos2.y - pos1.y).abs() as f32;
    x_diff + y_diff
}
