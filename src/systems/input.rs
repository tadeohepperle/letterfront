use bevy::{prelude::*, window::CursorGrabMode};

use crate::{
    components::LetterTile,
    constants::TILE_SIZE,
    resources::{CursorState, GrabbedLetter, GrabbedLetterResource},
};

pub struct InputSystemsPlugin;
impl Plugin for InputSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, send_cursor_state_events);
    }
}

fn send_cursor_state_events(
    windows: Query<&Window>,
    mut cursor_moved: EventReader<CursorMoved>,
    camera: Query<&Transform, With<Camera>>,
    mouse: Res<Input<MouseButton>>,

    mut cursor_state: ResMut<CursorState>,
) {
    /// credit: https://stackoverflow.com/questions/65396065/what-is-an-acceptable-approach-to-dragging-sprites-with-bevy-0-4
    fn cursor_to_world(window: &Window, cam_transform: &Transform, cursor_pos: Vec2) -> Vec2 {
        // get the size of the window
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let screen_pos = cursor_pos - size / 2.0;

        // apply the camera transform
        let out = cam_transform.compute_matrix() * screen_pos.extend(0.0).extend(1.0);
        Vec2::new(out.x, -out.y)
    }
    cursor_state.pressed = mouse.pressed(MouseButton::Left);

    if let Some(e) = cursor_moved.iter().last() {
        let screen_pos = e.position;
        let world_pos = cursor_to_world(
            windows.get_single().unwrap(),
            camera.get_single().unwrap(),
            e.position,
        );
        cursor_state.world_pos = world_pos;
        cursor_state.screen_pos = screen_pos;
    }
}
