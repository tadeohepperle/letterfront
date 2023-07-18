use std::fmt::format;

use bevy::{
    prelude::*,
    render::render_resource::AsBindGroupShaderType,
    text::Text2dBounds,
    utils::{HashMap, HashSet},
    window::{CursorGrabMode, PresentMode},
};

use letterfront::models::{array2d::Int2, corpus::Corpus, letterfield::Letterfield};
use letterfront::{events::EventsPlugin, resources::WordMatchesResource};
use rand::random;

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use letterfront::constants::*;
use letterfront::resources::{LetterfieldResource, ResourcesPlugin};
use letterfront::state::{IngameState, StateSystemsPlugin};
use letterfront::systems::{input::InputSystemsPlugin, setup::SetupSystemsPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample8)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                ..default()
            }),
            ..default()
        }))
        .add_state::<IngameState>()
        .add_plugins(EventsPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(StateSystemsPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(SetupSystemsPlugin)
        .add_plugins(InputSystemsPlugin)
        .add_systems(Update, egui_debug)
        .add_systems(Update, bevy::window::close_on_esc)
        // .add_systems(Update, cursor_grab_system)
        .run();
}

fn egui_debug(
    mut contexts: EguiContexts,
    ingame_state: Res<State<IngameState>>,
    letterfield: Res<LetterfieldResource>,
    word_matches: Res<WordMatchesResource>,
) {
    egui::Window::new("Info").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("IngameState: {:?}", ingame_state));
        ui.label(format!("Letterfield: \n {}", &letterfield.0));
        ui.label(format!("Matches: \n {:?}", &word_matches));
    });
}
