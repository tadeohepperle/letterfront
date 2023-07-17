use bevy::prelude::*;

use crate::{
    components::{HoverTile, LetterTile},
    constants::{TILE_GAP_FACTOR, TILE_SIZE, TILE_SPRITE_SIZE},
    models::{array2d::Int2, letterfield::Letterfield},
    resources::{FontAssets, LetterfieldResource},
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
    mut letter_tiles: Query<(Entity, &mut LetterTile)>,
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

    // let letter_HashSet<u32>
}

// fn update_letter_field_tiles(
//     mut commands: Commands,
//     letterfield: Res<LetterfieldResource>,
//     font_assets: Res<FontAssets>,
//     asset_server: Res<AssetServer>,
//     mut letter_tiles: Query<(Entity, &mut LetterTile)>,
// ) {
//     // id -> char, pos, exists
//     let mut letterfield_data: HashMap<u32, (char, Int2, bool)> = letterfield
//         .0
//         .iter()
//         .map(|(pos, (id, char))| (id, (char, pos, false)))
//         .collect();

//     let mut entities_out_of_date: Vec<Entity> = vec![];

//     // update all the entities (positions)
//     for (entity, mut letter_tile) in &mut letter_tiles {
//         if let Some((char, pos, exists)) = letterfield_data.get_mut(&letter_tile.id) {
//             *exists = true;
//             letter_tile.pos = *pos;
//             letter_tile.character = *char;
//         } else {
//             entities_out_of_date.push(entity)
//         }
//     }

//     // queue lettertiles that are out of date to be destroyed??
//     // todo!()

//     // create lettertiles that are not there yet.
//     for (id, (character, pos, _)) in letterfield_data
//         .iter()
//         .filter(|(_, (_, _, exists))| !exists)
//     {
//         create_new_letter_tile(
//             *id,
//             *character,
//             *pos,
//             &letterfield.0,
//             &font_assets,
//             &asset_server,
//             &mut commands,
//         );
//     }

//     // let letter_HashSet<u32>
// }

fn create_new_letter_tile(
    id: u32,
    character: char,
    pos: Int2,

    letterfield: &Letterfield,
    font_assets: &FontAssets,
    asset_server: &AssetServer,
    commands: &mut Commands,
) {
    let x = (pos.x as f32 - letterfield.width() as f32 / 2.0) * TILE_GAP_FACTOR * TILE_SIZE;
    let y = (pos.y as f32 - letterfield.height() as f32 / 2.0) * TILE_GAP_FACTOR * TILE_SIZE;

    // the parent:
    let tile = (
        SpatialBundle {
            transform: Transform {
                translation: Vec3 { x, y, z: 2.0 },
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
