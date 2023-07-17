use std::fmt::format;

use bevy::{
    prelude::*,
    render::render_resource::AsBindGroupShaderType,
    text::Text2dBounds,
    utils::{HashMap, HashSet},
    window::{CursorGrabMode, PresentMode},
};

use events::EventsPlugin;
use models::{array2d::Int2, corpus::Corpus, letterfield::Letterfield};
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

use constants::*;
use resources::ResourcesPlugin;
use state::IngameState;
use systems::MainSystemsPlugin;

use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub mod components;
pub mod constants;
pub mod events;
pub mod models;
pub mod resources;
pub mod state;
pub mod systems;

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
        .add_plugins(MainSystemsPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Update, egui_debug)
        // .add_systems(Update, cursor_grab_system)
        .run();
}

fn egui_debug(mut contexts: EguiContexts, ingame_state: Res<State<IngameState>>) {
    egui::Window::new("Info").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("IngameState: {:?}", ingame_state));
    });
}
