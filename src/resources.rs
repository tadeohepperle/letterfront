use bevy::{prelude::*, text::TextStyle, utils::HashSet};

use crate::{
    constants::{LETTERFIELD_SIZE, LETTERTILE_TEXT_SIZE},
    models::{
        array2d::Int2,
        corpus::Corpus,
        letterfield::{Letterfield, WordMatch},
    },
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GrabbedLetterResource>()
            .init_resource::<WordMatchesResource>()
            .init_resource::<CursorState>()
            .add_systems(PreStartup, load_corpus_and_init_letterfield)
            .add_systems(PreStartup, load_text_styles);
    }
}

#[derive(Debug, Resource)]
pub struct FontAssets {
    pub tile_text_style: TextStyle,
}

#[derive(Debug, Clone, Resource)]
pub struct CorpusResource(pub Corpus);

#[derive(Debug, Clone, Resource)]
pub struct LetterfieldResource(pub Letterfield);

#[derive(Debug, Clone, Resource, Default)]
pub struct WordMatchesResource {
    pub matches: Vec<WordMatch>,
    pub involved_ids: HashSet<u32>,
}

impl WordMatchesResource {
    pub fn set_matches(&mut self, matches: Vec<WordMatch>) {
        let involved_ids: HashSet<u32> = matches
            .iter()
            .flat_map(|m| m.tiles.iter().map(|(id, ..)| *id))
            .collect();
        self.matches = matches;
        self.involved_ids = involved_ids;
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }
}

/// u32 is the id in terms of the letterfield
#[derive(Debug, Clone, Resource, Default)]
pub struct GrabbedLetterResource(pub Option<GrabbedLetter>);

#[derive(Debug, Clone)]
pub struct GrabbedLetter {
    pub entity: Entity,
    pub id: u32,
    pub original_char_pos: Int2,
    pub new_char_pos: Int2,
    pub offset_to_cursor: Vec2,
}

#[derive(Debug, Clone, Resource, Default)]
pub struct CursorState {
    pub world_pos: Vec2,
    pub screen_pos: Vec2,
    pub pressed: bool,
}

// todo! LetterGrabStopEvent

// #[derive(Debug, Clone, Resource, Default)]
// pub struct LetterMatchesResource {}

fn load_text_styles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("NotoSerif-Bold.ttf");
    let tile_text_style = TextStyle {
        font: font.clone(),
        font_size: LETTERTILE_TEXT_SIZE,
        color: Color::BLACK,
    };
    commands.insert_resource(FontAssets { tile_text_style });
}

/// todo later: put this in loading stage
fn load_corpus_and_init_letterfield(mut commands: Commands) {
    let corpus = Corpus::from_txt_file("assets/english3000.txt").unwrap();
    let letterfield =
        Letterfield::random_with_no_matches(LETTERFIELD_SIZE.x, LETTERFIELD_SIZE.y, &corpus);
    commands.insert_resource(CorpusResource(corpus));
    commands.insert_resource(LetterfieldResource(letterfield));

    println!("loaded the corpus");
}
