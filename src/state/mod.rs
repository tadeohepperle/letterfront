pub mod grab;
pub mod inspect;

use bevy::prelude::*;

use bevy::prelude::States;

use self::{grab::IngameStateGrabPlugin, inspect::IngameStateInspectPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum IngameState {
    Inspect,
    Grab,
}

impl Default for IngameState {
    fn default() -> Self {
        IngameState::Inspect
    }
}

pub struct StateSystemsPlugin;

impl Plugin for StateSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IngameStateGrabPlugin)
            .add_plugins(IngameStateInspectPlugin);
    }
}
