#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    color::palettes::tailwind::{NEUTRAL_900, NEUTRAL_950},
    prelude::*,
    window::WindowResized,
};
use std::{collections::VecDeque, mem};

const GRID_CELLS: IVec2 = IVec2::new(40, 20);
const GRID_CENTER: IVec2 = IVec2::new(GRID_CELLS.x / 2, GRID_CELLS.y / 2);

#[derive(Default, Resource, Deref, DerefMut)]
struct CellSize(f32);

#[derive(Resource, Deref, DerefMut)]
struct TickTimer(Timer);

#[derive(Default, Resource)]
struct InputBuffer {
    deque: VecDeque<Direction>,
}

impl InputBuffer {
    fn push(&mut self, item: Direction) {
        if self.deque.len() == 4 {
            self.deque.pop_front();
        }
        self.deque.push_back(item);
    }

    fn pop(&mut self) -> Option<Direction> {
        self.deque.pop_front()
    }
}

#[derive(Component, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_vec(&self) -> Vec2 {
        match self {
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
            Direction::Up => Vec2::new(0.0, 1.0),
            Direction::Down => Vec2::new(0.0, -1.0),
        }
    }
}

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeBody;

#[derive(Component)]
struct SnakeTail;

#[derive(Component)]
struct Food;

#[derive(Component, Deref, DerefMut, PartialEq, Clone, Copy)]
struct Cell(Vec2);

fn setup_camera(mut commands: Commands, size: Res<CellSize>) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::from((
            (GRID_CELLS.as_vec2() * **size / 2.0) - **size / 2.0,
            0.0,
        ))),
    ));
}

fn setup_cell_size(window: Single<&Window>, mut size: ResMut<CellSize>) {
    **size = (Vec2::new(window.width(), window.height()) / GRID_CELLS.as_vec2()).min_element();
}

fn setup_grid(mut commands: Commands, size: Res<CellSize>) {
    for y in 0..GRID_CELLS.y {
        for x in 0..GRID_CELLS.x {
            let position = Vec2::new(x as f32, y as f32);
            commands.spawn((
                Cell(position),
                Sprite::from_color(
                    if (x + y) % 2 == 0 {
                        NEUTRAL_900
                    } else {
                        NEUTRAL_950
                    },
                    Vec2::ONE,
                ),
                Transform {
                    translation: Vec3::from((position * **size, 0.0)),
                    scale: Vec3::splat(**size),
                    ..Default::default()
                },
            ));
        }
    }
}

fn setup_snake(mut commands: Commands, asset_server: Res<AssetServer>, size: Res<CellSize>) {
    let texture = asset_server.load("cat.png");

    let position = GRID_CENTER.as_vec2();
    commands.spawn((
        SnakeHead,
        Direction::Right,
        Cell(position),
        Transform::from_translation(Vec3::from((position * **size, 0.0)))
            .with_scale(Vec3::splat(**size)),
        Sprite {
            image: texture.clone(),
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
    ));

    let position = position + Direction::Left.to_vec();
    commands.spawn((
        SnakeBody,
        Direction::Right,
        Cell(position),
        Transform::from_translation(Vec3::from((position * **size, 0.0)))
            .with_scale(Vec3::splat(**size)),
        Sprite {
            image: texture.clone(),
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
    ));

    let position = position + Direction::Left.to_vec();
    commands.spawn((
        SnakeTail,
        Direction::Right,
        Cell(position),
        Transform::from_translation(Vec3::from((position * 2.0 * **size, 0.0)))
            .with_scale(Vec3::splat(**size)),
        Sprite {
            image: texture.clone(),
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
    ));
}

fn setup_food(mut commands: Commands, asset_server: Res<AssetServer>, size: Res<CellSize>) {
    let mut position = Vec2::new(
        rand::random_range(0..GRID_CELLS.x) as f32,
        rand::random_range(0..GRID_CELLS.y) as f32,
    );

    while [
        GRID_CELLS.as_vec2(),
        GRID_CELLS.as_vec2() + Direction::Left.to_vec(),
        GRID_CELLS.as_vec2() + Direction::Left.to_vec() * 2.0,
    ]
    .contains(&position)
    {
        position = Vec2::new(
            rand::random_range(0..GRID_CELLS.x) as f32,
            rand::random_range(0..GRID_CELLS.y) as f32,
        );
    }

    commands.spawn((
        Food,
        Cell(position),
        Transform::from_translation(Vec3::from((position * **size, 0.0)))
            .with_scale(Vec3::splat(**size)),
        Sprite {
            image: asset_server.load("cat.png"),
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
    ));
}

fn offset_camera(
    mut camera: Single<&mut Transform, With<Camera>>,
    mut resize_reader: EventReader<WindowResized>,
    size: Res<CellSize>,
) {
    for _ in resize_reader.read() {
        camera.translation =
            Vec3::from(((GRID_CELLS.as_vec2() * **size / 2.0) - **size / 2.0, 0.0));
    }
}

fn resize_cells(
    mut resize_reader: EventReader<WindowResized>,
    mut cells: Query<(&mut Transform, &Cell)>,
    mut size: ResMut<CellSize>,
) {
    for e in resize_reader.read() {
        **size = (Vec2::new(e.width, e.height) / GRID_CELLS.as_vec2()).min_element() - 1.0;
        for (mut transform, cell) in &mut cells {
            transform.translation = Vec3::from((**cell * **size, 0.0));
            transform.scale = Vec3::splat(**size);
        }
    }
}

fn advance_tick_timer(time: Res<Time>, mut timer: ResMut<TickTimer>) {
    timer.tick(time.delta());
}

fn control_snake(
    mut head_dir: Single<&mut Direction, With<SnakeHead>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut input_buffer: ResMut<InputBuffer>,
    timer: Res<TickTimer>,
) {
    let mut new_dir = None;

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        new_dir = Some(Direction::Left);
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        new_dir = Some(Direction::Right);
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        new_dir = Some(Direction::Up);
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        new_dir = Some(Direction::Down);
    }

    if let Some(new_dir) = new_dir
        && new_dir != **head_dir
    {
        input_buffer.push(new_dir);
    }

    if timer.just_finished()
        && let Some(dir) = input_buffer.pop()
        && dir.to_vec() != -head_dir.to_vec()
    {
        *head_dir.as_mut() = dir;
    }
}

fn move_snake(
    head: Single<(&Direction, &mut Cell, &mut Transform), With<SnakeHead>>,
    mut body: Query<(&mut Cell, &mut Transform), (With<SnakeBody>, Without<SnakeHead>)>,
    tail: Single<
        (&mut Cell, &mut Transform),
        (With<SnakeTail>, Without<SnakeHead>, Without<SnakeBody>),
    >,
    size: Res<CellSize>,
    timer: Res<TickTimer>,
) {
    if !timer.just_finished() {
        return;
    }

    let (dir, mut cell, mut transform) = head.into_inner();
    let mut prev_pos = **cell;
    **cell += dir.to_vec();
    **cell += GRID_CELLS.as_vec2();
    **cell %= GRID_CELLS.as_vec2();
    transform.translation = Vec3::from((**cell * **size, 0.0));

    for (mut cell, mut transform) in &mut body {
        mem::swap(&mut prev_pos, &mut **cell);
        transform.translation = Vec3::from((**cell * **size, 0.0));
    }

    let (mut cell, mut transform) = tail.into_inner();
    mem::swap(&mut prev_pos, &mut **cell);
    transform.translation = Vec3::from((**cell * **size, 0.0));
}

fn food_consumption(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    head: Single<&Cell, With<SnakeHead>>,
    body: Query<&Cell, With<SnakeBody>>,
    tail: Single<(Entity, &Cell), With<SnakeTail>>,
    food: Single<
        (&mut Cell, &mut Transform),
        (
            With<Food>,
            Without<SnakeHead>,
            Without<SnakeBody>,
            Without<SnakeTail>,
        ),
    >,
    size: Res<CellSize>,
    timer: ResMut<TickTimer>,
) {
    if !timer.just_finished() {
        return;
    }

    let (mut food_cell, mut food_transform) = food.into_inner();
    let (tail_entity, tail_cell) = tail.into_inner();

    if **head == *food_cell {
        let mut new_pos = Vec2::new(
            rand::random_range(0..GRID_CELLS.x) as f32,
            rand::random_range(0..GRID_CELLS.y) as f32,
        );
        while ***head == new_pos
            || body.iter().any(|seg| **seg == new_pos)
            || **tail_cell == new_pos
        {
            new_pos = Vec2::new(
                rand::random_range(0..GRID_CELLS.x) as f32,
                rand::random_range(0..GRID_CELLS.y) as f32,
            );
        }

        **food_cell = new_pos;
        food_transform.translation = Vec3::from((new_pos * **size, 0.0));

        commands.entity(tail_entity).remove::<SnakeTail>();
        commands.entity(tail_entity).insert(SnakeBody);

        commands.spawn((
            SnakeTail,
            *tail_cell,
            Transform::from_translation(Vec3::from((**tail_cell * **size, 0.0)))
                .with_scale(Vec3::splat(**size)),
            Sprite {
                image: asset_server.load("cat.png"),
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup_cell_size,
                setup_camera,
                setup_grid,
                setup_snake,
                setup_food,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                resize_cells,
                offset_camera,
                advance_tick_timer,
                control_snake,
                move_snake,
                food_consumption,
            )
                .chain(),
        )
        .init_resource::<CellSize>()
        .insert_resource(TickTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .init_resource::<InputBuffer>()
        .run();
}
