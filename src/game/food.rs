use bevy::prelude::*;

use crate::game::{
    atlas::{SpriteAtlas, atlas_sprite},
    grid::cell,
};

#[derive(Component)]
pub struct Food;

pub fn food(position: Vec2, size: f32, atlas: &SpriteAtlas, atlas_index: usize) -> impl Bundle {
    (Food, cell(position, size), atlas_sprite(atlas, atlas_index))
}
