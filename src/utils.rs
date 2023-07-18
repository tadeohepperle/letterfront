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

/// cursor_pos is in world_pos space, bool = move is vertical:
pub fn cursor_pos_to_grabbed_tile_pos(
    cursor_pos: Vec2,
    w: usize,
    h: usize,
    original_char_pos: Int2, // when dragging started
    new_char_pos: Int2, // if already dragged a bit, this is different from original_char_pos, but always in same row/col
) -> (Vec2, Int2) {
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

    // todo: needle add grab threshold

    // set optimally restricted to the closest distance to mouse:
    let optimal_restricted = match (
        new_char_pos.x == original_char_pos.x,
        new_char_pos.y == original_char_pos.y,
    ) {
        (true, true) => {
            // not moved yet, the distance decides which direction it is gonna be:
            if vertical_only.distance(free_pos) < horizontal_only.distance(free_pos) {
                vertical_only
            } else {
                horizontal_only
            }
        }
        (true, false) => {
            // vertical movement:
            vertical_only
        }
        (false, true) => {
            // horizontal movement:
            horizontal_only
        }
        (false, false) => panic!("should not be reachable"),
    };

    //     (new_char_pos.x, new_char_pos.y) => {
    //         if vertical_only.distance(free_pos) < horizontal_only.distance(free_pos) {
    //   vertical_only
    // } else {
    //   horizontal_only
    // }
    // },

    // if new_char_pos == original_char_pos{
    //     // distance decides:
    //     if vertical_only.distance(free_pos) < horizontal_only.distance(free_pos) {
    //         vertical_only
    //     } else {
    //         horizontal_only
    //     };
    // } else if new_char_pos.x != original_char_pos

    // but, if already dragged in one direction that should be the optimally restricted one instead:

    let new_char_pos = world_pos_to_char_pos(optimal_restricted, w, h);

    (optimal_restricted, new_char_pos)
}
