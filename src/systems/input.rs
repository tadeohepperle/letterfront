use bevy::{prelude::*, window::CursorGrabMode};

use crate::{
    components::{HoverTile, LetterTile},
    constants::TILE_SIZE,
    events::CursorStateEvent,
    resources::{GrabbedLetter, GrabbedLetterResource},
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
    mut cursor_state_events: EventWriter<CursorStateEvent>,
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

    if let Some(e) = cursor_moved.iter().last() {
        let screen_pos = e.position;
        let world_pos = cursor_to_world(
            windows.get_single().unwrap(),
            camera.get_single().unwrap(),
            e.position,
        );
        cursor_state_events.send(CursorStateEvent {
            world_pos,
            screen_pos,
            pressed: mouse.pressed(MouseButton::Left),
        })
    }
}

// fn update_tile_hovered(
//     mut letter_tiles: Query<(Entity, &Transform, &mut HoverTile, &LetterTile)>,
//     mut cursor_state_events: EventReader<CursorStateEvent>,
//     mut windows: Query<&mut Window>,
//     mut grabbed_letter: ResMut<GrabbedLetterResource>,
// ) {
//     let Some(cursor_state) = cursor_state_events.iter().last() else {
//         return;
//     };

//     fn cursor_is_on_tile(cursor_world_pos: &Vec2, tile_transform: &Transform) -> bool {
//         let x_close = (tile_transform.translation.x - cursor_world_pos.x).abs()
//             <= tile_transform.scale.x * TILE_SIZE / 2.0;
//         let y_close = (tile_transform.translation.y - cursor_world_pos.y).abs()
//             <= tile_transform.scale.y * TILE_SIZE / 2.0;
//         x_close && y_close
//     }
//     let mut any_hovered = false;

//     let mut window = windows.get_single_mut().unwrap();

//     for (entity, transform, mut hover_tile, letter_tile) in &mut letter_tiles {
//         if cursor_is_on_tile(&cursor_state.world_pos, transform) {
//             // if !hover_tile.hovered {
//             //     dbg!(("Hover Enter", letter_tile));
//             // }
//             hover_tile.hovered = true;
//             any_hovered = true;

//             if cursor_state.pressed && grabbed_letter.0.is_none() {
//                 dbg!(("Start dragging", letter_tile));
//                 grabbed_letter.0 = Some(GrabbedLetter {
//                     id: letter_tile.id,
//                     entity,
//                     offset_to_cursor: cursor_state.world_pos - transform.translation.truncate(),
//                 });
//                 window.cursor.visible = false;
//             }
//         } else {
//             // if hover_tile.hovered {
//             //     dbg!(("Hover Exit", letter_tile));
//             // }
//             hover_tile.hovered = false;
//         }
//     }

//     if any_hovered {
//         window.cursor.icon = CursorIcon::Hand
//     } else {
//         window.cursor.icon = CursorIcon::Default
//     }
// }
