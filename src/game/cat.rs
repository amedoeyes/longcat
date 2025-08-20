use std::mem;

use bevy::prelude::*;

use crate::{
    game::{
        GameSystems, InGame,
        atlas::{AtlasSprite, SpriteAtlas, atlas_sprite},
        food::Food,
        grid::{Cell, CellSize, GRID_CELLS, cell},
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (move_cat, open_mouth_if_near_food, consume_food, handle_hit)
            .chain()
            .in_set(GameSystems::FixedUpdate),
    );
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Debug)]
pub struct CatHead;

#[derive(Component, Debug)]
pub struct CatBody;

#[derive(Component, Debug)]
pub struct CatTail;

impl Direction {
    pub fn to_vec(self) -> Vec2 {
        match self {
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
            Direction::Up => Vec2::new(0.0, 1.0),
            Direction::Down => Vec2::new(0.0, -1.0),
        }
    }

    pub fn to_quat(self) -> Quat {
        match self {
            Direction::Right => Quat::IDENTITY,
            Direction::Left => Quat::from_rotation_z(std::f32::consts::PI),
            Direction::Up => Quat::from_rotation_z(std::f32::consts::PI / 2.0),
            Direction::Down => Quat::from_rotation_z(-std::f32::consts::PI / 2.0),
        }
    }
}

pub fn cat_head(
    position: Vec2,
    direction: Direction,
    size: f32,
    atlas: &SpriteAtlas,
    atlas_index: usize,
) -> impl Bundle {
    (
        CatHead,
        cat_segment(position, direction, size, atlas, atlas_index),
    )
}

pub fn cat_body(
    position: Vec2,
    direction: Direction,
    size: f32,
    atlas: &SpriteAtlas,
    atlas_index: usize,
) -> impl Bundle {
    (
        CatBody,
        cat_segment(position, direction, size, atlas, atlas_index),
    )
}

pub fn cat_tail(
    position: Vec2,
    direction: Direction,
    size: f32,
    atlas: &SpriteAtlas,
    atlas_index: usize,
) -> impl Bundle {
    (
        CatTail,
        cat_segment(position, direction, size, atlas, atlas_index),
    )
}

fn cat_segment(
    position: Vec2,
    direction: Direction,
    size: f32,
    atlas: &SpriteAtlas,
    atlas_index: usize,
) -> impl Bundle {
    (
        direction,
        cell(position, size),
        atlas_sprite(atlas, atlas_index),
    )
}

fn move_cat(
    head: Single<(&mut Cell, &mut Direction, &mut Transform, &mut Sprite), With<CatHead>>,
    mut body: Query<
        (&mut Cell, &mut Direction, &mut Transform, &mut Sprite),
        (With<CatBody>, Without<CatHead>),
    >,
    tail: Single<
        (&mut Cell, &mut Direction, &mut Transform, &mut Sprite),
        (With<CatTail>, Without<CatHead>, Without<CatBody>),
    >,
    size: Res<CellSize>,
) {
    let (mut cell, mut dir, mut transform, mut sprite) = head.into_inner();

    let mut prev_dir = *dir;
    let mut next_dir = prev_dir;
    let mut prev_cell = **cell;

    *dir = next_dir;
    **cell = (**cell + next_dir.to_vec()).rem_euclid(GRID_CELLS.as_vec2());
    transform.translation = Vec3::from((**cell * **size, transform.translation.z));
    transform.rotation = dir.to_quat();
    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        atlas.index = if atlas.index == AtlasSprite::Head1.into() {
            AtlasSprite::Head2.into()
        } else {
            AtlasSprite::Head1.into()
        };
    }

    for (mut cell, mut dir, mut transform, mut sprite) in &mut body {
        mem::swap(&mut prev_cell, &mut **cell);
        let cross = dir.to_vec().perp_dot(next_dir.to_vec());
        sprite.texture_atlas.as_mut().unwrap().index = if cross == 0.0 {
            AtlasSprite::Body1.into()
        } else {
            AtlasSprite::Body2.into()
        };
        prev_dir = *dir;
        *dir = next_dir;
        next_dir = prev_dir;
        transform.translation = Vec3::from((**cell * **size, transform.translation.z));
        transform.rotation = dir.to_quat();
        if cross > 0.0 {
            transform.rotation *= Quat::from_rotation_x(std::f32::consts::PI)
        }
    }

    let (mut cell, mut dir, mut transform, mut sprite) = tail.into_inner();
    **cell = prev_cell;
    *dir = next_dir;
    transform.translation = Vec3::from((**cell * **size, transform.translation.z));
    transform.rotation = dir.to_quat();
    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        atlas.index = if atlas.index == AtlasSprite::Tail1.into() {
            AtlasSprite::Tail2.into()
        } else {
            AtlasSprite::Tail1.into()
        };
    }
}

fn open_mouth_if_near_food(
    head: Single<(&Cell, &Direction, &mut Sprite), With<CatHead>>,
    food: Single<&Cell, With<Food>>,
) {
    let (head_cell, head_dir, mut head_sprite) = head.into_inner();

    let dir_vec = head_dir.to_vec();
    if [dir_vec, dir_vec.perp(), -dir_vec.perp()]
        .map(|dir| (**head_cell + dir).rem_euclid(GRID_CELLS.as_vec2()))
        .contains(*food)
    {
        if let Some(atlas) = head_sprite.texture_atlas.as_mut() {
            atlas.index = if atlas.index == AtlasSprite::Head1.into() {
                AtlasSprite::Head3.into()
            } else {
                AtlasSprite::Head4.into()
            };
        }
    }
}

fn handle_hit(
    mut commands: Commands,
    head: Single<(&mut Cell, &Direction, &mut Transform, &mut Sprite), With<CatHead>>,
    rest: Query<(Entity, &Cell), (Or<(With<CatBody>, With<CatTail>)>, Without<CatHead>)>,
    size: Res<CellSize>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let (mut head_cell, head_dir, mut head_transform, mut head_sprite) = head.into_inner();
    if rest.iter().any(|(_, cell)| *cell == *head_cell) {
        commands.entity(rest.iter().next().unwrap().0).despawn();
        **head_cell = (**head_cell + (-head_dir.to_vec())).rem_euclid(GRID_CELLS.as_vec2());
        head_transform.translation =
            Vec3::from((**head_cell * **size, head_transform.translation.z));
        if let Some(atlas) = head_sprite.texture_atlas.as_mut() {
            atlas.index = AtlasSprite::Head5.into();
        }
        next_screen.set(Screen::Over)
    }
}

fn consume_food(
    mut commands: Commands,
    head: Single<&Cell, With<CatHead>>,
    body: Query<&Cell, With<CatBody>>,
    tail: Single<(&Cell, &Direction, &Transform, &Sprite), With<CatTail>>,
    food: Single<
        (&mut Cell, &mut Transform),
        (
            With<Food>,
            Without<CatHead>,
            Without<CatBody>,
            Without<CatTail>,
        ),
    >,
    size: Res<CellSize>,
) {
    let (mut food_cell, mut food_transform) = food.into_inner();
    let (tail_cell, tail_dir, tail_transform, tail_sprite) = tail.into_inner();

    // TODO: create a list of empty cells instead and if the list is empty switch to a win state or someting
    if **head == *food_cell {
        let mut new_pos = Vec2::new(
            rand::random_range(0..GRID_CELLS.x) as f32,
            rand::random_range(0..GRID_CELLS.y) as f32,
        );

        while ***head == new_pos
            || body.iter().any(|cell| **cell == new_pos)
            || **tail_cell == new_pos
        {
            new_pos = Vec2::new(
                rand::random_range(0..GRID_CELLS.x) as f32,
                rand::random_range(0..GRID_CELLS.y) as f32,
            );
        }

        **food_cell = new_pos;
        food_transform.translation = Vec3::from((new_pos * **size, food_transform.translation.z));

        commands.spawn((
            StateScoped(InGame::True),
            CatBody,
            *tail_cell,
            *tail_dir,
            *tail_transform,
            tail_sprite.clone(),
        ));
    }
}
