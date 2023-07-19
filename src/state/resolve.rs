use bevy::{
    prelude::*,
    transform::{self, commands},
    utils::{HashMap, HashSet},
    window::CursorGrabMode,
};

use crate::{
    components::{Dying, Falling, LetterTile},
    constants::{GRAVITY_ACCELERATION, RESOLVE_DURATION},
    models::{
        array2d::Int2,
        letterfield::{self, LetterReplacement, Letterfield, WordMatch},
    },
    resources::{CorpusResource, FontAssets, LetterfieldResource, WordMatchesResource},
    systems::setup::create_letter_tile,
    utils::char_pos_to_world_pos,
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
            .add_systems(Update, animate_dying_tiles)
            .add_systems(Update, destroy_dying_tiles_out_of_bounds);
    }
}

#[derive(Debug, Clone, Resource)]
pub struct ResolvingTimer {
    timer: Timer,
}

// #[derive(Debug, Clone, Event)]
// pub struct StartResolving {
//     matches: Vec<WordMatch>,
//     replacements: Vec<Replacement>,
// }

// #[derive(Debug, Clone, Event)]
// pub struct EndResolving;

pub fn start_resolving(
    // mut start_resolving_tx: EventWriter<StartResolving>,
    mut letterfield: ResMut<LetterfieldResource>,
    corpus: Res<CorpusResource>,
    tiles: Query<(Entity, &LetterTile)>,
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    let letterfield_resolve = letterfield
        .0
        .find_word_matches_and_fill_spaces_randomly(&corpus.0);

    let removed_ids: HashSet<u32> = replacements.iter().map(|r| r.old.0).collect();

    for (entity, tile) in &tiles {
        if removed_ids.contains(&tile.id) {
            commands
                .entity(entity)
                .remove::<LetterTile>()
                .insert(Dying::random()); // todo random direction
        }
    }
    // add a NewLetterTile Component for all new tile,
    for LetterReplacement {
        pos,
        old: _,
        new: (new_id, new_char),
    } in replacements
    {
        // id: u32,
        // character: char,
        // pos: Int2,
        // letterfield: &Letterfield,
        // font_assets: &FontAssets,
        // asset_server: &AssetServer,
        // commands: &mut Commands,
        // additional: impl Component,
        let target_world_pos =
            char_pos_to_world_pos(pos, letterfield.0.width(), letterfield.0.height());
        let mut initial_world_pos = target_world_pos;
        initial_world_pos.y += 200.0;
        create_letter_tile(
            new_id,
            new_char,
            pos,
            &letterfield.0,
            &font_assets,
            &asset_server,
            &mut commands,
            Falling { target_world_pos },
            Some(initial_world_pos),
        );
    }
    // insert a timer that drives a system for animating old and new tiles, at the end resolve again.
    commands.insert_resource(ResolvingTimer {
        timer: Timer::new(RESOLVE_DURATION, TimerMode::Once),
    })
}

pub fn animate_falling_tiles(
    mut falling_tiles: Query<(Entity, &mut Transform, &LetterTile, &Falling)>,
) {
    // let all falling tiles fall down at constant speed:
    // falling
}

pub fn animate_dying_tiles(
    // mut falling_tiles: Query<(Entity, &mut Transform, &LetterTile, &Falling)>,
    mut dying_tiles: Query<(&mut Transform, &mut Dying)>,
    time: Res<Time>,
) {
    for (mut transform, mut dying) in &mut dying_tiles {
        dying.speed_down += GRAVITY_ACCELERATION * time.delta_seconds();
        transform.translation +=
            (dying.direction * dying.speed - Vec2::Y * dying.speed_down).extend(0.0);
    }
}

pub fn destroy_dying_tiles_out_of_bounds(
    dying_tiles: Query<(Entity, &Transform), With<Dying>>,
    mut commands: Commands,
) {
    for (entity, transform) in &dying_tiles {
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn()
        }
        println!("Despawn entity {:?}", entity)
    }
}
