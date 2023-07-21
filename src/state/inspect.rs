use bevy::prelude::*;

use crate::{
    components::{HoverableTile, LetterTile},
    constants::TILE_SIZE,
    resources::{CursorState, GrabbedLetter, GrabbedLetterResource},
};

use super::IngameState;
pub struct IngameStateInspectPlugin;

impl Plugin for IngameStateInspectPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            update_hover_state.run_if(in_state(IngameState::Inspect)),
        )
        .add_systems(Update, update_hover_colors);
    }
}

pub fn update_hover_colors(
    q_tiles: Query<(&HoverableTile, &Children)>,
    mut q_tile_sprites: Query<&mut Sprite>,
) {
    // todo!(): lerp!
    for (hover_tile, children) in &q_tiles {
        for child in children {
            if let Ok(mut sprite) = q_tile_sprites.get_mut(*child) {
                if hover_tile.hovered {
                    sprite.color = Color::VIOLET;
                } else {
                    sprite.color = Color::WHITE;
                }
            }
        }
    }
}

fn update_hover_state(
    mut letter_tiles: Query<(Entity, &Transform, &mut HoverableTile, &LetterTile)>,
    mut cursor_state: Res<CursorState>,
    mut windows: Query<&mut Window>,
    mut grabbed_letter: ResMut<GrabbedLetterResource>,
    mut next_state: ResMut<NextState<IngameState>>,
) {
    fn cursor_is_on_tile(cursor_world_pos: &Vec2, tile_transform: &Transform) -> bool {
        let x_close = (tile_transform.translation.x - cursor_world_pos.x).abs()
            <= tile_transform.scale.x * TILE_SIZE / 2.0;
        let y_close = (tile_transform.translation.y - cursor_world_pos.y).abs()
            <= tile_transform.scale.y * TILE_SIZE / 2.0;
        x_close && y_close
    }
    let mut any_hovered = false;

    let mut window = windows.get_single_mut().unwrap();

    for (entity, transform, mut hoverable, letter_tile) in &mut letter_tiles {
        if cursor_is_on_tile(&cursor_state.world_pos, transform) {
            // if !hover_tile.hovered {
            //     dbg!(("Hover Enter", letter_tile));
            // }
            hoverable.hovered = true;
            any_hovered = true;

            if cursor_state.pressed && grabbed_letter.0.is_none() {
                grabbed_letter.0 = Some(GrabbedLetter {
                    id: letter_tile.id,
                    entity,
                    offset_to_cursor: cursor_state.world_pos - transform.translation.truncate(),
                    original_char_pos: letter_tile.pos,
                    new_char_pos: letter_tile.pos,
                });
                window.cursor.visible = false;
                next_state.set(IngameState::Grab);
            }
        } else {
            // if hover_tile.hovered {
            //     dbg!(("Hover Exit", letter_tile));
            // }
            hoverable.hovered = false;
        }
    }

    if any_hovered {
        window.cursor.icon = CursorIcon::Hand
    } else {
        window.cursor.icon = CursorIcon::Default
    }
}
