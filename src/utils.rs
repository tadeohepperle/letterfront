use bevy::prelude::Vec2;

use crate::{
    constants::{TILE_GAP_FACTOR, TILE_SIZE},
    models::array2d::Int2,
};

pub fn char_pos_to_world_pos(char_pos: Int2, w: usize, h: usize) -> Vec2 {
    let x = (char_pos.x as f32 - w as f32 / 2.0) * TILE_GAP_FACTOR * TILE_SIZE;
    let y = (char_pos.y as f32 - h as f32 / 2.0) * TILE_GAP_FACTOR * TILE_SIZE;
    Vec2 { x, y }
}

pub fn world_pos_to_char_pos(world_pos: Vec2, w: usize, h: usize) -> Int2 {
    let x = (world_pos.x / (TILE_GAP_FACTOR * TILE_SIZE)) + w as f32 / 2.0;
    let y = (world_pos.y / (TILE_GAP_FACTOR * TILE_SIZE)) + h as f32 / 2.0;
    Int2 {
        x: x.round() as usize,
        y: y.round() as usize,
    }
}

pub enum Direction {
    Horizontal,
    Vertical,
}

/// cursor_pos is in world_pos space, bool = move is vertical:
pub fn cursor_pos_to_grabbed_tile_pos(
    cursor_pos: Vec2,
    w: usize,
    h: usize,
    original_char_pos: Int2,
) -> (Vec2, Int2, Option<Direction>) {
    let mut free_pos = cursor_pos;
    // restrict the movement by the rect:
    let Vec2 { x: x_min, y: y_min } = char_pos_to_world_pos(Int2 { x: 0, y: 0 }, w, h);
    let Vec2 { x: x_max, y: y_max } = char_pos_to_world_pos(Int2 { x: w - 1, y: h - 1 }, w, h);
    free_pos.x = free_pos.x.max(x_min).min(x_max);
    free_pos.y = free_pos.y.max(y_min).min(y_max);
    // either move horizontally or vertically, not both:
    let original_tile_world_pos = char_pos_to_world_pos(original_char_pos, w, h);

    let vertical_only = Vec2 {
        x: original_tile_world_pos.x,
        y: free_pos.y,
    };
    let horizontal_only = Vec2 {
        x: free_pos.x,
        y: original_tile_world_pos.y,
    };

    let (optimal_restricted, is_vertical) =
        if vertical_only.distance(free_pos) < horizontal_only.distance(free_pos) {
            (vertical_only, true)
        } else {
            (horizontal_only, false)
        };

    let new_char_pos = world_pos_to_char_pos(optimal_restricted, w, h);

    let direction = match (new_char_pos == original_char_pos, is_vertical) {
        (true, _) => None,
        (false, true) => Some(Direction::Vertical),
        (false, false) => Some(Direction::Horizontal),
    };

    (optimal_restricted, new_char_pos, direction)
}
