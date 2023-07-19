pub mod grab;
pub mod inspect;
pub mod resolve;

use bevy::prelude::*;

use bevy::prelude::States;

use self::{
    grab::IngameStateGrabPlugin, inspect::IngameStateInspectPlugin,
    resolve::IngameStateResolvePlugin,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum IngameState {
    Inspect,
    Grab,
    Resolve,
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
            .add_plugins(IngameStateInspectPlugin)
            .add_plugins(IngameStateResolvePlugin);
    }
}
