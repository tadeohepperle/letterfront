use bevy::{
    prelude::*,
    transform::{self, commands},
    utils::HashMap,
    window::CursorGrabMode,
};

use crate::{
    components::{HoverTile, LetterTile},
    models::{array2d::Int2, letterfield},
    resources::{
        CorpusResource, CursorState, GrabbedLetterResource, LetterfieldResource,
        WordMatchesResource,
    },
    utils::{char_pos_to_world_pos, cursor_pos_to_grabbed_tile_pos, world_pos_to_char_pos},
};

use super::{inspect::update_hover_colors, IngameState};

pub struct IngameStateGrabPlugin;

impl Plugin for IngameStateGrabPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            move_grabbed_letter_to_cursor.run_if(in_state(IngameState::Grab)),
        )
        .add_systems(
            PreUpdate,
            exit_grabbed_state_if_not_mouse_pressed.run_if(in_state(IngameState::Grab)),
        )
        .add_systems(
            Update,
            move_letter_tiles_to_correct_positions, // not only in grab state.
        )
        .add_systems(
            Update,
            update_word_matches_colors.after(update_hover_colors),
        );
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
        // window.cursor.grab_mode = CursorGrabMode::Locked;
        // window.set_cursor_position(Some(Vec2 { x: 0., y: 0. })); // todo!() needle
        next_state.set(IngameState::Inspect);
    }
}

fn move_grabbed_letter_to_cursor(
    mut cursor_state: ResMut<CursorState>,
    mut grabbed_letter: ResMut<GrabbedLetterResource>,
    mut tiles: Query<(&mut Transform, &mut LetterTile)>,
    mut letterfield: ResMut<LetterfieldResource>,
    mut word_matches: ResMut<WordMatchesResource>,
    corpus: Res<CorpusResource>,
) {
    let Some(grabbed_letter) = &mut grabbed_letter.0 else {
        return;
    };

    // let word_pos_rounded = char_pos_to_world_pos(
    //     world_pos_to_char_pos(
    //         cursor_state.world_pos,
    //         letterfield.0.width(),
    //         letterfield.0.height(),
    //     ),
    //     letterfield.0.width(),
    //     letterfield.0.height(),
    // );

    let (grabbed_tile_pos, new_char_pos) = cursor_pos_to_grabbed_tile_pos(
        cursor_state.world_pos,
        letterfield.0.width(),
        letterfield.0.height(),
        grabbed_letter.original_char_pos,
        grabbed_letter.new_char_pos,
    );

    // set the position of the letter physically:
    let (mut transform, _) = tiles.get_mut(grabbed_letter.entity).unwrap();
    transform.translation = grabbed_tile_pos.extend(5.0); // todo!() needle: plus offset

    // check if the grabbed letter has a new char pos:
    if grabbed_letter.new_char_pos != new_char_pos {
        let old_char_pos = grabbed_letter.new_char_pos;
        grabbed_letter.new_char_pos = new_char_pos;
        // move the char in the lettergrid:
        letterfield.0.move_letter(old_char_pos, new_char_pos);
        // todo: needle recalculate matches
        word_matches.set_matches(letterfield.0.find_word_matches(&corpus.0));
        // update all letter_tiles (data only, not transform):
        let tile_data_positions: HashMap<u32, Int2> = letterfield
            .0
            .iter()
            .map(|(pos, (id, _))| {
                // dbg!(id, pos);
                (id, pos)
            })
            .collect();
        for (_, mut letter_tile) in &mut tiles {
            // dbg!(&letter_tile.id);
            letter_tile.pos = tile_data_positions[&letter_tile.id];
        }
    }
}

fn move_letter_tiles_to_correct_positions(
    mut tiles: Query<(&mut Transform, &LetterTile)>,
    mut grabbed_letter: ResMut<GrabbedLetterResource>,
    letterfield: Res<LetterfieldResource>,
    time: Res<Time>,
) {
    let (w, h) = letterfield.0.dimensions();
    for (mut transform, letter_tile) in &mut tiles {
        let world_pos_target = char_pos_to_world_pos(letter_tile.pos, w, h).extend(2.0);
        const LERP_SPEED: f32 = 5.0;
        let lerp_factor = time.delta_seconds() * LERP_SPEED;
        transform.translation =
            world_pos_target * lerp_factor + (1.0 - lerp_factor) * transform.translation;
    }
}

pub fn update_word_matches_colors(
    tiles: Query<(&LetterTile, &Children)>,
    mut tile_sprites: Query<&mut Sprite>,
    word_matches: Res<WordMatchesResource>,
) {
    for (letter_tile, children) in &tiles {
        for child in children {
            if let Ok(mut sprite) = tile_sprites.get_mut(*child) {
                let count = *word_matches.id_counts.get(&letter_tile.id).unwrap_or(&0);

                sprite.color = match count {
                    0 => Color::WHITE,
                    1 => Color::LIME_GREEN,
                    2 => Color::GREEN,
                    3 => Color::DARK_GREEN,
                    _ => Color::PURPLE,
                };
            }
        }
    }

    // // todo!(): lerp!
    // for (hover_tile, children) in &q_tiles {
    //     for child in children {
    //         if let Ok(mut sprite) = q_tile_sprites.get_mut(*child) {
    //             if hover_tile.hovered {
    //                 sprite.color = Color::VIOLET;
    //             } else {
    //                 sprite.color = Color::WHITE;
    //             }
    //         }
    //     }
    // }
}
