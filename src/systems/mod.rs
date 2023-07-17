pub mod animation;
pub mod input;
pub mod setup;

use bevy::prelude::*;

use self::{
    animation::AnimationSystemsPlugin, input::InputSystemsPlugin, setup::SetupSystemsPlugin,
};
pub struct MainSystemsPlugin;

impl Plugin for MainSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SetupSystemsPlugin,
            AnimationSystemsPlugin,
            InputSystemsPlugin,
        ));
    }
}
