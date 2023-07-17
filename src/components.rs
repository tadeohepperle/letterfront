use bevy::{prelude::*, text::TextStyle};

use crate::models::array2d::Int2;

#[derive(Debug, Clone, Component)]
pub struct HoverTile {
    pub hovered: bool,
}

#[derive(Debug, Clone, Component)]
pub struct LetterTile {
    pub id: u32,
    pub character: char,
    pub pos: Int2,
}
