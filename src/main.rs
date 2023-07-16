use bevy::prelude::*;

use models::{array2d::Int2, corpus::Corpus, letterfield::Letterfield};
use rand::random;

pub mod models;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(PreStartup, load_corpus_and_init_letterfield)
        .add_systems(PreStartup, load_text_styles)
        .add_systems(Startup, init_letter_field_tiles)
        .add_systems(Startup, init_camera)
        .run()
}

#[derive(Debug, Clone, Resource)]
pub struct CorpusResource {
    inner: Corpus,
}

#[derive(Debug, Clone, Resource)]
pub struct LetterfieldResource {
    inner: Letterfield,
}

fn load_text_styles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("BalooBhai2-Bold.ttf");
    let tile_text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::BLACK,
    };
    commands.insert_resource(FontAssets { tile_text_style });
}

/// todo later: put this in loading stage
fn load_corpus_and_init_letterfield(mut commands: Commands) {
    let corpus = Corpus::from_txt_file("assets/english3000.txt").unwrap();
    let letterfield =
        Letterfield::random_with_no_matches(LETTERFIELD_SIZE.x, LETTERFIELD_SIZE.y, &corpus);
    commands.insert_resource(CorpusResource { inner: corpus });
    commands.insert_resource(LetterfieldResource { inner: letterfield });

    println!("loaded the corpus");
}

fn init_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn init_letter_field_tiles(
    mut commands: Commands,
    letterfield: Res<LetterfieldResource>,
    font_assets: Res<FontAssets>,
) {
    // root node that is entire screen:
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::BEIGE.into(),
            ..default()
        })
        .with_children(|root_node| {
            root_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(0.0),
                        height: Val::Px(0.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|middle_node| {
                    for (c, pos) in letterfield.inner.chars_and_positions() {
                        spawn_letter_tile(middle_node, c, pos, &font_assets);
                    }
                });
        });
}

const TILE_SIZE: Val = Val::Px(50.0);
const LETTERFIELD_SIZE: Int2 = Int2 { x: 10, y: 8 };

fn spawn_letter_tile(
    middle_node: &mut ChildBuilder,
    character: char,
    pos: Int2,
    font_assets: &FontAssets,
) {
    let tile_rect = NodeBundle {
        style: Style {
            width: TILE_SIZE,
            height: TILE_SIZE,
            // border: UiRect::all(Val::Px(20.)),
            position_type: PositionType::Absolute,
            left: TILE_SIZE * 1.1 * (pos.x as f32 - LETTERFIELD_SIZE.x as f32 / 2.0),
            bottom: TILE_SIZE * 1.1 * (pos.y as f32 - LETTERFIELD_SIZE.y as f32 / 2.0),
            // bottom: Val::Percent(50.),
            ..default()
        },
        background_color: Color::rgba(random(), random(), random(), 0.1).into(),
        ..default()
    };

    let text_node = Text2dBundle {
        text: Text::from_section("rotation", font_assets.tile_text_style.clone())
            .with_alignment(TextAlignment::Center),
        ..default()
    };

    println!("spawn letter {character} {pos:?}");

    middle_node
        .spawn((tile_rect, LetterTile { character, pos }))
        .with_children(|square| {
            square.spawn(text_node);
        });
}

#[derive(Debug, Clone, Component)]
struct LetterTile {
    character: char,
    pos: Int2,
}

#[derive(Resource)]
struct FontAssets {
    tile_text_style: TextStyle,
}
