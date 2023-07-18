// use bevy::prelude::*;

// use crate::{
//     components::{HoverTile, LetterTile},
//     events::CursorStateEvent,
//     resources::GrabbedLetterResource,
// };

// pub struct AnimationSystemsPlugin;
// impl Plugin for AnimationSystemsPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, lerp_hovered_letters_color);
//         app.add_systems(Update, move_grabbed_letter_to_cursor);
//     }
// }

// fn lerp_hovered_letters_color(
//     q_tiles: Query<(&HoverTile, &Children)>,
//     mut q_tile_sprites: Query<&mut Sprite>,
// ) {
//     // todo!(): lerp!
//     for (hover_tile, children) in &q_tiles {
//         for child in children {
//             if let Ok(mut sprite) = q_tile_sprites.get_mut(*child) {
//                 if hover_tile.hovered {
//                     sprite.color = Color::VIOLET;
//                 } else {
//                     sprite.color = Color::WHITE;
//                 }
//             }
//         }
//     }
// }
