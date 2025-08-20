use bevy::prelude::*;

use crate::game::{
    GameState, GameSystems, InGame,
    atlas::{AtlasSprite, SpriteAtlas},
    cat::{CatBody, CatHead, CatTail, Direction, cat_body, cat_head, cat_tail},
    food::{Food, food},
    grid::{CellSize, GRID_CELLS, GRID_CENTER, cell},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnExit(GameState::None),
        spawn_level.in_set(GameSystems::Spawn),
    );

    app.add_systems(
        OnTransition {
            exited: GameState::Over,
            entered: GameState::Run,
        },
        reset_level,
    );
}

fn spawn_level(mut commands: Commands, size: Res<CellSize>, atlas: Res<SpriteAtlas>) {
    for y in 0..GRID_CELLS.y {
        for x in 0..GRID_CELLS.x {
            let position = Vec2::new(x as f32, y as f32);
            commands.spawn((
                StateScoped(InGame::True),
                cell(position, **size),
                Sprite::from_color(
                    if (x + y) % 2 == 0 {
                        Color::srgb_u8(0x10, 0x10, 0x10)
                    } else {
                        Color::srgb_u8(0x20, 0x20, 0x20)
                    },
                    Vec2::ONE,
                ),
            ));
        }
    }

    let cat_positions = [
        GRID_CENTER.as_vec2(),
        GRID_CENTER.as_vec2() + Direction::Left.to_vec(),
        GRID_CENTER.as_vec2() + Direction::Left.to_vec() * 2.0,
    ];
    let cat_directions = [Direction::Right, Direction::Right, Direction::Right];
    let cat_atlas_indicies = [
        AtlasSprite::Head1.into(),
        AtlasSprite::Body1.into(),
        AtlasSprite::Tail1.into(),
    ];

    commands.spawn((
        StateScoped(InGame::True),
        cat_head(
            cat_positions[0],
            cat_directions[0],
            **size,
            &atlas,
            cat_atlas_indicies[0],
        ),
    ));

    commands.spawn((
        StateScoped(InGame::True),
        cat_body(
            cat_positions[1],
            cat_directions[1],
            **size,
            &atlas,
            cat_atlas_indicies[1],
        ),
    ));

    commands.spawn((
        StateScoped(InGame::True),
        cat_tail(
            cat_positions[2],
            cat_directions[2],
            **size,
            &atlas,
            cat_atlas_indicies[2],
        ),
    ));

    let mut position = Vec2::new(
        rand::random_range(0..GRID_CELLS.x) as f32,
        rand::random_range(0..GRID_CELLS.y) as f32,
    );

    while cat_positions.contains(&position) {
        position = Vec2::new(
            rand::random_range(0..GRID_CELLS.x) as f32,
            rand::random_range(0..GRID_CELLS.y) as f32,
        );
    }

    commands.spawn((
        StateScoped(InGame::True),
        food(position, **size, &atlas, AtlasSprite::Fish.into()),
    ));
}

fn reset_level(
    mut commands: Commands,
    snake: Query<Entity, Or<(With<CatHead>, With<CatBody>, With<CatTail>)>>,
    food: Single<Entity, With<Food>>,
) {
    for entity in snake.iter() {
        commands.entity(entity).despawn();
    }
    commands.entity(*food).despawn();
    commands.run_system_cached(spawn_level);
}
