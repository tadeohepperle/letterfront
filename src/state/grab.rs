use bevy::prelude::*;

use crate::{
    components::{HoverTile, LetterTile},
    events::CursorStateEvent,
    resources::GrabbedLetterResource,
};

use super::IngameState;

pub struct IngameStateGrabPlugin;

impl Plugin for IngameStateGrabPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}

fn move_grabbed_letter_to_cursor(
    mut cursor_state: EventReader<CursorStateEvent>,
    grabbed_letter: Res<GrabbedLetterResource>,
    mut q_tiles: Query<&mut Transform, With<LetterTile>>,
) {
    let Some(grabbed_letter) = &grabbed_letter.0 else {
        return;
    };

    let Some(cursor_state) = cursor_state.iter().last() else {
        return;
    };

    let mut transform = q_tiles.get_mut(grabbed_letter.entity).unwrap();
    transform.translation = cursor_state.world_pos.extend(5.0); // todo plus offset
}
