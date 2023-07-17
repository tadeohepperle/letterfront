pub mod grab;
pub mod inspect;

use bevy::prelude::*;

use bevy::prelude::States;

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

        // app.add_plugins((
        //     SetupSystemsPlugin,
        //     AnimationSystemsPlugin,
        //     InputSystemsPlugin,
        // ));
    }
}
