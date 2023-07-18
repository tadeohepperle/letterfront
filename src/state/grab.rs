use bevy::{prelude::*, transform::commands};

use crate::{
    components::{HoverTile, LetterTile},
    resources::{CursorState, GrabbedLetterResource, LetterfieldResource},
    utils::{char_pos_to_world_pos, cursor_pos_to_grabbed_tile_pos, world_pos_to_char_pos},
};

use super::IngameState;

pub struct IngameStateGrabPlugin;

impl Plugin for IngameStateGrabPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            move_grabbed_letter_to_cursor.run_if(in_state(IngameState::Grab)),
        )
        .add_systems(PreUpdate, exit_grabbed_state_if_not_mouse_pressed);
    }
}

fn exit_grabbed_state_if_not_mouse_pressed(
    mut cursor_state: ResMut<CursorState>,
    mut next_state: ResMut<NextState<IngameState>>,
    mut windows: Query<&mut Window>,
    mut grabbed_letter: ResMut<GrabbedLetterResource>,
) {
    let mut window = windows.get_single_mut().unwrap();

    if !cursor_state.pressed {
        // todo: update dropped item???
        grabbed_letter.0 = None;
        window.cursor.visible = true;
        next_state.set(IngameState::Inspect);
    }
}

fn move_grabbed_letter_to_cursor(
    mut cursor_state: ResMut<CursorState>,
    grabbed_letter: Res<GrabbedLetterResource>,
    mut tiles: Query<&mut Transform, With<LetterTile>>,
    letterfield: Res<LetterfieldResource>,
) {
    let Some(grabbed_letter) = &grabbed_letter.0 else {
        return;
    };

    let word_pos_rounded = char_pos_to_world_pos(
        world_pos_to_char_pos(
            cursor_state.world_pos,
            letterfield.0.width(),
            letterfield.0.height(),
        ),
        letterfield.0.width(),
        letterfield.0.height(),
    );

    let (grabbed_tile_pos, new_char_pos, move_direction) = cursor_pos_to_grabbed_tile_pos(
        cursor_state.world_pos,
        letterfield.0.width(),
        letterfield.0.height(),
        grabbed_letter.original_pos,
    );

    let mut transform = tiles.get_mut(grabbed_letter.entity).unwrap();
    transform.translation = grabbed_tile_pos.extend(5.0); // todo plus offset
}
