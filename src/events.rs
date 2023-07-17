use bevy::{prelude::*, text::TextStyle};
pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CursorStateEvent>();
    }
}

#[derive(Debug, Clone, Event)]
pub struct CursorStateEvent {
    pub world_pos: Vec2,
    pub screen_pos: Vec2,
    pub pressed: bool,
}
