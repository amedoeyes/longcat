use bevy::prelude::*;

use crate::game::{GameState, GameSystems};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SpriteAtlas>();
    app.add_systems(
        OnExit(GameState::None),
        setup_texture_atlas.in_set(GameSystems::Resources),
    );
}

#[derive(Debug)]
pub enum AtlasSprite {
    Head1 = 0,
    Head2 = 1,
    Head3 = 2,
    Head4 = 3,
    Head5 = 4,
    Body1 = 6,
    Body2 = 8,
    Tail1 = 10,
    Tail2 = 11,
    Fish = 12,
}

impl From<AtlasSprite> for usize {
    fn from(val: AtlasSprite) -> Self {
        val as usize
    }
}

#[derive(Resource, Default, Debug)]
pub struct SpriteAtlas {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn atlas_sprite(atlas: &SpriteAtlas, index: usize) -> impl Bundle {
    (Sprite {
        image: atlas.image.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: atlas.layout.clone(),
            index,
        }),
        custom_size: Some(Vec2::ONE),
        ..default()
    },)
}

fn setup_texture_atlas(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut atlas: ResMut<SpriteAtlas>,
) {
    atlas.image = asset_server.load("atlas.png");
    atlas.layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        2,
        7,
        None,
        None,
    ));
}
