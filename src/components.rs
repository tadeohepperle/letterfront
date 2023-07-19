use bevy::{prelude::*, text::TextStyle};
use rand::random;

use crate::models::array2d::Int2;

#[derive(Debug, Clone, Component)]
pub struct HoverTile {
    pub hovered: bool,
}

#[derive(Debug, Clone, Component)]
pub struct LetterTile {
    pub id: u32,
    pub character: char,
    pub pos: Int2,
}

#[derive(Debug, Clone, Component)]
pub struct Falling {
    pub target_world_pos: Vec2,
}

#[derive(Debug, Clone, Component)]
pub struct Dying {
    pub torque: f32,
    pub direction: Vec2,
    pub speed: f32,
    pub speed_down: f32,
}

impl Dying {
    pub fn random() -> Self {
        Self {
            torque: random(),
            direction: Vec2 {
                x: random::<f32>() * 2.0 - 1.0,
                y: random::<f32>() * 2.0,
            }
            .normalize(), //random::<f32>() * 360.0
            speed: random(),
            speed_down: 0.0,
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct EmptyComponent;
