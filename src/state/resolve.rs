use bevy::{
    prelude::*,
    transform::{self, commands},
    utils::{HashMap, HashSet},
    window::CursorGrabMode,
};

use crate::{
    components::{FallingLetter, HoverableTile, LetterTile},
    constants::{FALLING_TIME_PER_UNIT, GRAVITY_ACCELERATION, RESOLVE_DURATION},
    models::{
        array2d::Int2,
        letterfield::{self, Letterfield, WordMatch},
    },
    resources::{CorpusResource, FontAssets, LetterfieldResource, WordMatchesResource},
    systems::setup::create_letter_tile,
    utils::{char_pos_to_world_pos, char_pos_to_world_pos_i, AnimationDriver},
};

use super::IngameState;
pub struct IngameStateResolvePlugin;

impl Plugin for IngameStateResolvePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(IngameState::Resolve), start_resolving)
            .add_systems(
                Update,
                animate_falling_tiles.run_if(in_state(IngameState::Resolve)),
            )
            // .add_systems(Update, animate_dying_tiles)
            // .add_systems(Update, destroy_dying_tiles_out_of_bounds);

            ;
    }
}

// #[derive(Debug, Clone, Event)]
// pub struct StartResolving {
//     matches: Vec<WordMatch>,
//     replacements: Vec<Replacement>,
// }

// #[derive(Debug, Clone, Event)]
// pub struct EndResolving;

pub fn start_resolving(
    mut letterfield: ResMut<LetterfieldResource>,
    corpus: Res<CorpusResource>,
    mut tiles: Query<(Entity, &mut LetterTile)>,
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<IngameState>>,
) {
    println!("start resolve");
    let resolve = letterfield
        .0
        .find_word_matches_and_fill_spaces_randomly(&corpus.0);

    if resolve.is_empty() {
        println!("transition back to inspect");
        next_state.set(IngameState::Inspect);
        return;
    }

    // new letters get spawned in
    for (id, (pos, char)) in resolve.new_letters {
        let start_world_pos = char_pos_to_world_pos_i(
            (pos.x as isize, -5),
            letterfield.0.width(),
            letterfield.0.height(),
        );
        let target_world_pos =
            char_pos_to_world_pos(pos, letterfield.0.width(), letterfield.0.height());

        create_letter_tile(
            id,
            char,
            pos,
            &letterfield.0,
            &font_assets,
            &asset_server,
            &mut commands,
            FallingLetter {
                start_world_pos,
                target_world_pos,
                time: 0.0,
                target_time: (pos.y as f32 + 5.0 as f32) * FALLING_TIME_PER_UNIT,
            },
            Some(start_world_pos),
        );
    }

    for (entity, mut letter_tile) in &mut tiles {
        if let Some((from, to, char)) = resolve.moving_letters.get(&letter_tile.id) {
            assert_eq!(letter_tile.character, *char);
            letter_tile.pos = *to;
            // let entity fall

            let start_world_pos =
                char_pos_to_world_pos(*from, letterfield.0.width(), letterfield.0.height());
            let target_world_pos =
                char_pos_to_world_pos(*to, letterfield.0.width(), letterfield.0.height());
            commands
                .entity(entity)
                .remove::<HoverableTile>()
                .insert(FallingLetter {
                    start_world_pos,
                    target_world_pos,
                    time: 0.0,
                    target_time: (from.y as f32 - to.y as f32).abs() * FALLING_TIME_PER_UNIT,
                });
        }

        if let Some((pos, char)) = resolve.old_letters.get(&letter_tile.id) {
            println!("despawned {char} at {pos:?}");
            // todo!() needle
            commands.entity(entity).despawn_recursive()
        }
    }
}

pub fn animate_falling_tiles(
    mut falling_tiles: Query<(Entity, &mut Transform, &mut FallingLetter)>,
    time: Res<Time>,
    mut commands: Commands,
    // just for forwarding to resolve again:
    mut letterfield: ResMut<LetterfieldResource>,
    corpus: Res<CorpusResource>,
    mut tiles: Query<(Entity, &mut LetterTile)>,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<IngameState>>,
) {
    let mut all_finished = true;
    for (entity, mut transform, mut falling) in &mut falling_tiles {
        let finished = falling.drive(&mut transform, time.delta_seconds());
        if finished {
            // transition to a normal tile: give it hovertile, remove falling tile
            commands
                .entity(entity)
                .insert(HoverableTile { hovered: false })
                .remove::<FallingLetter>();
        } else {
            all_finished = false;
        }
    }

    if all_finished {
        // transition back to inspect state, or resolve again
        println!("transition all finished");
        start_resolving(
            letterfield,
            corpus,
            tiles,
            commands,
            font_assets,
            asset_server,
            next_state,
        )
    }
}

// pub fn animate_dying_tiles(
//     // mut falling_tiles: Query<(Entity, &mut Transform, &LetterTile, &Falling)>,
//     mut dying_tiles: Query<(&mut Transform, &mut Dying)>,
//     time: Res<Time>,
// ) {
//     for (mut transform, mut dying) in &mut dying_tiles {
//         dying.speed_down += GRAVITY_ACCELERATION * time.delta_seconds();
//         transform.translation +=
//             (dying.direction * dying.speed - Vec2::Y * dying.speed_down).extend(0.0);
//     }
// }

// pub fn destroy_dying_tiles_out_of_bounds(
//     dying_tiles: Query<(Entity, &Transform), With<Dying>>,
//     mut commands: Commands,
// ) {
//     for (entity, transform) in &dying_tiles {
//         if transform.translation.y < -300.0 {
//             commands.entity(entity).despawn()
//         }
//         println!("Despawn entity {:?}", entity)
//     }
// }
