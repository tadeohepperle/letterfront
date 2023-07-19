use std::time::Duration;

use crate::models::array2d::Int2;

pub const TILE_SIZE: f32 = 64.0;
pub const LETTERFIELD_SIZE: Int2 = Int2 { x: 10, y: 8 };
pub const LETTERTILE_TEXT_SIZE: f32 = 64.0;
pub const TILE_SPRITE_SIZE: f32 = 256.;
pub const TILE_GAP_FACTOR: f32 = 1.1;
pub const RESOLVE_DURATION: Duration = Duration::from_millis(1000);
pub const GRAVITY_ACCELERATION: f32 = 9.81;
