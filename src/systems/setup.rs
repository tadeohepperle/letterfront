use bevy::prelude::*;

use crate::{
    components::{HoverTile, LetterTile},
    constants::{TILE_GAP_FACTOR, TILE_SIZE, TILE_SPRITE_SIZE},
    models::{array2d::Int2, letterfield::Letterfield},
    resources::{FontAssets, LetterfieldResource},
    utils::char_pos_to_world_pos,
};

pub struct SetupSystemsPlugin;
impl Plugin for SetupSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera)
            .add_systems(Startup, setup_letter_field_tiles);
    }
}

fn init_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_letter_field_tiles(
    mut commands: Commands,
    letterfield: Res<LetterfieldResource>,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    // create lettertiles
    for (pos, (id, character)) in letterfield.0.iter() {
        create_new_letter_tile(
            id,
            character,
            pos,
            &letterfield.0,
            &font_assets,
            &asset_server,
            &mut commands,
        );
    }
}

fn create_new_letter_tile(
    id: u32,
    character: char,
    pos: Int2,

    letterfield: &Letterfield,
    font_assets: &FontAssets,
    asset_server: &AssetServer,
    commands: &mut Commands,
) {
    let word_pos = char_pos_to_world_pos(pos, letterfield.width(), letterfield.height());
    // the parent:
    let tile = (
        SpatialBundle {
            transform: Transform {
                translation: word_pos.extend(2.0),
                ..default()
            },
            ..default()
        },
        LetterTile { character, pos, id },
        HoverTile { hovered: false },
    );

    // child 1: the sprite
    let tile_rect = SpriteBundle {
        sprite: Sprite { ..default() },
        transform: Transform {
            scale: Vec3::splat(TILE_SIZE / TILE_SPRITE_SIZE),
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 2.,
            },
            ..default()
        },
        texture: asset_server.load("tile.png"),
        ..default()
    };

    // child 2: the text

    let tile_text = Text2dBundle {
        text: Text::from_section(character.to_string(), font_assets.tile_text_style.clone()),
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 3.,
            },
            scale: Vec3::splat(1.0),
            ..default()
        },
        ..default()
    };

    // spawn the 3 elements:
    commands.spawn(tile).with_children(|tile| {
        tile.spawn(tile_text);
        tile.spawn(tile_rect);
    });
}
