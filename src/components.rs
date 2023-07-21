use bevy::{prelude::*, text::TextStyle};
use rand::random;

use crate::{models::array2d::Int2, utils::AnimationDriver};

#[derive(Debug, Clone, Component)]
pub struct LetterTile {
    pub id: u32,
    pub character: char,
    pub pos: Int2,
}

#[derive(Debug, Clone, Component)]
pub struct FallingLetter {
    pub time: f32,
    pub target_time: f32,
    pub start_world_pos: Vec2,
    pub target_world_pos: Vec2,
}

#[derive(Debug, Clone, Component)]
pub struct FadingLetter {}

#[derive(Debug, Clone, Component)]
pub struct HoverableTile {
    pub hovered: bool,
}

// pub enum

impl AnimationDriver for FallingLetter {
    type ActOn = Transform;

    fn drive(&mut self, act_on: &mut Self::ActOn, delta_seconds: f32) -> bool {
        self.time += delta_seconds;
        self.time = self.time.min(self.target_time);

        let Vec2 { x, y } = self.start_world_pos
            + (self.target_world_pos - self.start_world_pos) * (self.time / self.target_time);
        act_on.translation.x = x;
        act_on.translation.y = y;

        self.time == self.target_time
    }
}

// #[derive(Debug, Clone, Component)]
// pub struct Dying {
//     pub torque: f32,
//     pub direction: Vec2,
//     pub speed: f32,
//     pub speed_down: f32,
// }

// impl Dying {
//     pub fn random() -> Self {
//         Self {
//             torque: random(),
//             direction: Vec2 {
//                 x: random::<f32>() * 2.0 - 1.0,
//                 y: random::<f32>() * 2.0,
//             }
//             .normalize(), //random::<f32>() * 360.0
//             speed: random(),
//             speed_down: 0.0,
//         }
//     }
// }

#[derive(Debug, Clone, Component)]
pub struct EmptyComponent;
