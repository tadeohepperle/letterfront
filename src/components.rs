use std::f32::consts::PI;

use bevy::{prelude::*, sprite::COLOR_MATERIAL_SHADER_HANDLE, text::TextStyle};
use rand::random;

use crate::{
    constants::FALLING_SPEED_PER_10_BLOCKS, models::array2d::Int2, utils::AnimationDriver,
};

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

impl AnimationDriver for FallingLetter {
    type ActOn = Transform;

    fn drive(&mut self, act_on: &mut Self::ActOn, delta_seconds: f32) -> bool {
        self.time += delta_seconds * FALLING_SPEED_PER_10_BLOCKS;
        self.time = self.time.min(self.target_time);

        let Vec2 { x, y } = self.start_world_pos
            + (self.target_world_pos - self.start_world_pos) * (self.time / self.target_time);
        act_on.translation.x = x;
        act_on.translation.y = y;

        self.time == self.target_time
    }
}

#[derive(Debug, Clone, Component)]
pub struct FadingLetter {
    pub time: f32,
    pub explode_direction: Vec2,
    pub down_speed: f32,
}

impl FadingLetter {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            explode_direction: Vec2 {
                x: random::<f32>().abs(),
                y: random::<f32>() * 2.0,
            } * 200.0,
            down_speed: 0.0,
        }
    }
}

impl FadingLetter {
    pub fn drive(&mut self, transform: &mut Transform, delta_seconds: f32) -> bool {
        // sprite.color = Color::AQUAMARINE;
        transform.translation.z = 10.0;
        self.time += delta_seconds;
        // growing:
        const GROWTIME: f32 = 0.2;
        const GROW_TO: f32 = 1.1;
        let grow_progress = self.time.clamp(0.0, GROWTIME) / GROWTIME;
        let scale = (1.0 - grow_progress) * 1.0 + grow_progress * GROW_TO;
        transform.scale.x = scale;
        transform.scale.y = scale;
        // shake:
        const SHAKETIME: f32 = 0.5;
        let shake_progress =
            (self.time.clamp(GROWTIME, GROWTIME + SHAKETIME) - GROWTIME) / SHAKETIME;
        let shake = (shake_progress * 2.0 * PI * 1.0).sin() * 0.2;
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., shake);
        // explode:
        if self.time > GROWTIME + SHAKETIME {
            self.down_speed += delta_seconds * 1000.0;
            transform.translation += (self.explode_direction * delta_seconds
                + Vec2 {
                    x: 0.0,
                    y: -self.down_speed * delta_seconds,
                })
            .extend(0.0);
        }

        const DEATHTIME: f32 = 10.0;
        self.time > DEATHTIME
    }
}

#[derive(Debug, Clone, Component)]
pub struct HoverableTile {
    pub hovered: bool,
}

// pub enum

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
