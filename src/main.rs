#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    input::{common_conditions::input_just_pressed, keyboard::KeyboardInput},
    input_focus::{
        FocusedInput, InputDispatchPlugin, InputFocus,
        directional_navigation::{
            DirectionalNavigation, DirectionalNavigationMap, DirectionalNavigationPlugin,
        },
    },
    math::CompassOctant,
    prelude::*,
    window::WindowResized,
};
use std::{collections::VecDeque, mem};

const GRID_CELLS: IVec2 = IVec2::new(20, 10);
const GRID_CENTER: IVec2 = IVec2::new(GRID_CELLS.x / 2, GRID_CELLS.y / 2);

#[derive(Debug)]
enum AtlasSprite {
    Head1 = 0,
    Head2 = 1,
    Head3 = 2,
    Head4 = 3,
    Body1 = 4,
    Body2 = 6,
    Tail1 = 8,
    Tail2 = 9,
    Fish = 10,
}

#[derive(Default, Resource, Deref, DerefMut)]
struct CellSize(f32);

#[derive(Resource, Deref, DerefMut)]
struct TickTimer(Timer);

#[derive(Default, Resource)]
struct Atlas {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Debug, Default, Resource)]
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

#[derive(Debug, Component, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_vec(self) -> Vec2 {
        match self {
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
            Direction::Up => Vec2::new(0.0, 1.0),
            Direction::Down => Vec2::new(0.0, -1.0),
        }
    }

    fn to_quat(self) -> Quat {
        match self {
            Direction::Right => Quat::IDENTITY,
            Direction::Left => Quat::from_rotation_z(std::f32::consts::PI),
            Direction::Up => Quat::from_rotation_z(std::f32::consts::PI / 2.0),
            Direction::Down => Quat::from_rotation_z(-std::f32::consts::PI / 2.0),
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

#[derive(Debug, Component, Deref, DerefMut, PartialEq, Clone, Copy)]
struct Cell(Vec2);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_gameplay_camera(mut camera: Single<&mut Transform, With<Camera2d>>, size: Res<CellSize>) {
    camera.translation = Vec3::from(((GRID_CELLS.as_vec2() * **size / 2.0) - **size / 2.0, 0.0));
}

fn setup_cell_size(window: Single<&Window>, mut size: ResMut<CellSize>) {
    **size = (Vec2::new(window.width(), window.height()) / GRID_CELLS.as_vec2()).min_element();
}

fn setup_texture_atlas(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut atlas: ResMut<Atlas>,
) {
    atlas.image = asset_server.load("atlas.png");
    atlas.layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        2,
        6,
        None,
        None,
    ));
}

fn setup_grid(mut commands: Commands, size: Res<CellSize>) {
    for y in 0..GRID_CELLS.y {
        for x in 0..GRID_CELLS.x {
            let position = Vec2::new(x as f32, y as f32);
            commands.spawn((
                StateScoped(AppState::Gameplay),
                Cell(position),
                Sprite::from_color(
                    if (x + y) % 2 == 0 {
                        Color::srgb_u8(0x10, 0x10, 0x10)
                    } else {
                        Color::srgb_u8(0x20, 0x20, 0x20)
                    },
                    Vec2::ONE,
                ),
                Transform {
                    translation: Vec3::from((position * **size, 0.0)),
                    scale: Vec3::splat(**size),
                    ..default()
                },
            ));
        }
    }
}

fn setup_snake(mut commands: Commands, size: Res<CellSize>, atlas: Res<Atlas>) {
    let position = GRID_CENTER.as_vec2();
    commands.spawn((
        StateScoped(AppState::Gameplay),
        SnakeHead,
        Direction::Right,
        Cell(position),
        Transform::from_translation(Vec3::from((position * **size, 0.0)))
            .with_scale(Vec3::splat(**size)),
        Sprite {
            image: atlas.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas.layout.clone(),
                index: AtlasSprite::Head1 as usize,
            }),
            custom_size: Some(Vec2::ONE),
            ..default()
        },
    ));

    let position = position + Direction::Left.to_vec();
    commands.spawn((
        StateScoped(AppState::Gameplay),
        SnakeBody,
        Direction::Right,
        Cell(position),
        Transform::from_translation(Vec3::from((position * **size, 0.0)))
            .with_scale(Vec3::splat(**size)),
        Sprite {
            image: atlas.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas.layout.clone(),
                index: AtlasSprite::Body1 as usize,
            }),
            custom_size: Some(Vec2::ONE),
            ..default()
        },
    ));

    let position = position + Direction::Left.to_vec();
    commands.spawn((
        StateScoped(AppState::Gameplay),
        SnakeTail,
        Direction::Right,
        Cell(position),
        Transform::from_translation(Vec3::from((position * **size, 0.0)))
            .with_scale(Vec3::splat(**size)),
        Sprite {
            image: atlas.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas.layout.clone(),
                index: AtlasSprite::Tail1 as usize,
            }),
            custom_size: Some(Vec2::ONE),
            ..default()
        },
    ));
}

fn setup_food(mut commands: Commands, size: Res<CellSize>, atlas: Res<Atlas>) {
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
        StateScoped(AppState::Gameplay),
        Food,
        Cell(position),
        Transform::from_translation(Vec3::from((position * **size, 0.0)))
            .with_scale(Vec3::splat(**size)),
        Sprite {
            image: atlas.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas.layout.clone(),
                index: AtlasSprite::Fish as usize,
            }),
            custom_size: Some(Vec2::ONE),
            ..default()
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

fn control_snake(keyboard_input: Res<ButtonInput<KeyCode>>, mut input_buffer: ResMut<InputBuffer>) {
    let mut new_dir = None;

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        new_dir = Some(Direction::Left);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        new_dir = Some(Direction::Right);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        new_dir = Some(Direction::Up);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        new_dir = Some(Direction::Down);
    }

    if let Some(new_dir) = new_dir {
        input_buffer.push(new_dir);
    }
}

fn move_snake(
    head: Single<(&mut Cell, &mut Direction, &mut Transform, &mut Sprite), With<SnakeHead>>,
    mut body: Query<
        (&mut Cell, &mut Direction, &mut Transform, &mut Sprite),
        (With<SnakeBody>, Without<SnakeHead>),
    >,
    tail: Single<
        (&mut Cell, &mut Direction, &mut Transform, &mut Sprite),
        (With<SnakeTail>, Without<SnakeHead>, Without<SnakeBody>),
    >,
    size: Res<CellSize>,
    timer: Res<TickTimer>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    if !timer.just_finished() {
        return;
    }

    let (mut cell, mut dir, mut transform, mut sprite) = head.into_inner();

    let mut prev_dir = *dir;
    let mut next_dir = prev_dir;
    let mut prev_cell = **cell;

    while let Some(new_dir) = input_buffer.pop() {
        if new_dir == *dir || -new_dir.to_vec() == dir.to_vec() {
            continue;
        }

        let next_cell = (**cell + new_dir.to_vec()).rem_euclid(GRID_CELLS.as_vec2());
        if body.iter().any(|(cell, _, _, _)| **cell == next_cell) || **tail.0 == next_cell {
            continue;
        }

        next_dir = new_dir;
        break;
    }

    *dir = next_dir;
    **cell = (**cell + next_dir.to_vec()).rem_euclid(GRID_CELLS.as_vec2());
    transform.translation = Vec3::from((**cell * **size, 0.0));
    transform.rotation = dir.to_quat();
    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        atlas.index = if atlas.index == AtlasSprite::Head1 as usize {
            AtlasSprite::Head2 as usize
        } else {
            AtlasSprite::Head1 as usize
        };
    }

    for (mut cell, mut dir, mut transform, mut sprite) in &mut body {
        mem::swap(&mut prev_cell, &mut **cell);
        let cross = dir.to_vec().perp_dot(next_dir.to_vec());
        sprite.texture_atlas.as_mut().unwrap().index = if cross == 0.0 {
            AtlasSprite::Body1 as usize
        } else {
            AtlasSprite::Body2 as usize
        };
        prev_dir = *dir;
        *dir = next_dir;
        next_dir = prev_dir;
        transform.translation = Vec3::from((**cell * **size, 0.0));
        transform.rotation = dir.to_quat();
        if cross > 0.0 {
            transform.rotation *= Quat::from_rotation_x(std::f32::consts::PI)
        }
    }

    let (mut cell, mut dir, mut transform, mut sprite) = tail.into_inner();
    **cell = prev_cell;
    *dir = next_dir;
    transform.translation = Vec3::from((**cell * **size, 0.0));
    transform.rotation = dir.to_quat();
    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        atlas.index = if atlas.index == AtlasSprite::Tail1 as usize {
            AtlasSprite::Tail2 as usize
        } else {
            AtlasSprite::Tail1 as usize
        };
    }
}

fn open_mouth(
    head: Single<(&Cell, &Direction, &mut Sprite), With<SnakeHead>>,
    food: Single<&Cell, With<Food>>,
    timer: ResMut<TickTimer>,
) {
    if !timer.just_finished() {
        return;
    }

    let (head_cell, head_dir, mut head_sprite) = head.into_inner();

    let dir_vec = head_dir.to_vec();
    if [dir_vec, dir_vec.perp(), -dir_vec.perp()]
        .map(|dir| (**head_cell + dir).rem_euclid(GRID_CELLS.as_vec2()))
        .contains(*food)
    {
        if let Some(atlas) = head_sprite.texture_atlas.as_mut() {
            atlas.index = if atlas.index == AtlasSprite::Head1 as usize {
                AtlasSprite::Head3 as usize
            } else {
                AtlasSprite::Head4 as usize
            };
        }
    }
}

fn consume_food(
    mut commands: Commands,
    head: Single<&Cell, With<SnakeHead>>,
    body: Query<&Cell, With<SnakeBody>>,
    tail: Single<(&Cell, &Direction, &Transform, &Sprite), With<SnakeTail>>,
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
        food_transform.translation = Vec3::from((new_pos * **size, 0.0));

        commands.spawn((
            StateScoped(AppState::Gameplay),
            SnakeBody,
            *tail_cell,
            *tail_dir,
            *tail_transform,
            tail_sprite.clone(),
        ));
    }
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[states(scoped_entities)]
enum AppState {
    #[default]
    Menu,
    Gameplay,
}

#[derive(SubStates, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[source(AppState = AppState::Gameplay)]
#[states(scoped_entities)]
enum GameplayState {
    #[default]
    Running,
    Paused,
    Over,
}

#[derive(Component)]
struct MenuButton;

fn navigate_menu_up(mut dir_nav: DirectionalNavigation) {
    dir_nav.navigate(CompassOctant::North).unwrap();
}

fn navigate_menu_down(mut dir_nav: DirectionalNavigation) {
    dir_nav.navigate(CompassOctant::South).unwrap();
}

fn highlight_focused_menu_button(
    mut query: Query<(Entity, &mut TextColor), With<MenuButton>>,
    input_focus: Res<InputFocus>,
) {
    for (entity, mut text_color) in query.iter_mut() {
        if input_focus.0 == Some(entity) {
            text_color.0 = Color::srgb(1.0, 0.0, 0.0);
        } else {
            text_color.0 = Color::WHITE;
        }
    }
}

fn create_menu_button(label: &str) -> impl Bundle {
    (
        MenuButton,
        Text::new(label),
        TextColor(Color::WHITE),
        TextFont::from_font_size(40.0),
    )
}

fn create_menu_root_node() -> impl Bundle {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

fn setup_main_menu(
    mut commands: Commands,
    mut dir_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    commands
        .spawn((StateScoped(AppState::Menu), create_menu_root_node()))
        .with_children(|parent| {
            let start_btn = parent
                .spawn(create_menu_button("Start"))
                .observe(
                    |_: Trigger<FocusedInput<KeyboardInput>>,
                     keyboard_input: Res<ButtonInput<KeyCode>>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        if keyboard_input.just_pressed(KeyCode::Enter) {
                            next_state.set(AppState::Gameplay);
                        }
                    },
                )
                .id();

            let quit_btn = parent
                .spawn(create_menu_button("Quit"))
                .observe(
                    |_: Trigger<FocusedInput<KeyboardInput>>,
                     keyboard_input: Res<ButtonInput<KeyCode>>,
                     mut event_writer: EventWriter<AppExit>| {
                        if keyboard_input.just_pressed(KeyCode::Enter) {
                            event_writer.write(AppExit::Success);
                        }
                    },
                )
                .id();

            dir_nav_map.add_looping_edges(&[start_btn, quit_btn], CompassOctant::South);
            input_focus.set(start_btn);
        });
}

fn setup_pause_menu(
    mut commands: Commands,
    mut dir_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    commands
        .spawn((
            StateScoped(GameplayState::Paused),
            create_menu_root_node(),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
        ))
        .with_children(|parent| {
            let resume_btn = parent
                .spawn(create_menu_button("Resume"))
                .observe(
                    |_: Trigger<FocusedInput<KeyboardInput>>,
                     keyboard_input: Res<ButtonInput<KeyCode>>,
                     mut next_state: ResMut<NextState<GameplayState>>| {
                        if keyboard_input.just_pressed(KeyCode::Enter) {
                            next_state.set(GameplayState::Running);
                        }
                    },
                )
                .id();

            let quit_btn = parent
                .spawn(create_menu_button("Quit"))
                .observe(
                    |_: Trigger<FocusedInput<KeyboardInput>>,
                     keyboard_input: Res<ButtonInput<KeyCode>>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        if keyboard_input.just_pressed(KeyCode::Enter) {
                            next_state.set(AppState::Menu);
                        }
                    },
                )
                .id();

            dir_nav_map.add_looping_edges(&[resume_btn, quit_btn], CompassOctant::South);
            input_focus.set(resume_btn);
        });
}

fn toggle_pause(
    current_state: Res<State<GameplayState>>,
    mut next_state: ResMut<NextState<GameplayState>>,
) {
    if *current_state != GameplayState::Paused {
        next_state.set(GameplayState::Paused)
    } else {
        next_state.set(GameplayState::Running)
    }
}

fn main() {
    let mut app = App::default();

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        InputDispatchPlugin,
        DirectionalNavigationPlugin,
    ));

    app.init_state::<AppState>();
    app.add_sub_state::<GameplayState>();

    app.init_resource::<CellSize>();
    app.init_resource::<Atlas>();
    app.init_resource::<InputBuffer>();
    app.insert_resource(ClearColor(Color::BLACK));
    app.insert_resource(TickTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

    app.add_systems(Startup, setup_camera);

    app.add_systems(Startup, setup_main_menu.run_if(in_state(AppState::Menu))); // delete this in 0.17
    app.add_systems(OnEnter(AppState::Menu), setup_main_menu);
    app.add_systems(
        Update,
        (
            (
                navigate_menu_up.run_if(input_just_pressed(KeyCode::ArrowUp)),
                navigate_menu_down.run_if(input_just_pressed(KeyCode::ArrowDown)),
            ),
            highlight_focused_menu_button.run_if(resource_changed::<InputFocus>),
        )
            .chain()
            .run_if(in_state(AppState::Menu)),
    );

    app.add_systems(OnEnter(GameplayState::Paused), setup_pause_menu);
    app.add_systems(
        Update,
        (
            (
                navigate_menu_up.run_if(input_just_pressed(KeyCode::ArrowUp)),
                navigate_menu_down.run_if(input_just_pressed(KeyCode::ArrowDown)),
            ),
            highlight_focused_menu_button.run_if(resource_changed::<InputFocus>),
        )
            .chain()
            .run_if(in_state(GameplayState::Paused)),
    );

    app.add_systems(
        Update,
        toggle_pause.run_if(
            in_state(GameplayState::Running)
                .or(in_state(GameplayState::Paused))
                .and(input_just_pressed(KeyCode::Space)),
        ),
    );

    app.add_systems(
        OnEnter(AppState::Gameplay),
        (
            setup_texture_atlas,
            setup_cell_size,
            setup_gameplay_camera,
            setup_grid,
            setup_snake,
            setup_food,
        )
            .chain(),
    );

    app.add_systems(
        Update,
        (
            resize_cells,
            offset_camera.run_if(resource_changed::<CellSize>),
            (
                advance_tick_timer,
                control_snake,
                move_snake,
                open_mouth,
                consume_food,
            )
                .chain()
                .run_if(in_state(GameplayState::Running)),
        )
            .chain()
            .run_if(in_state(AppState::Gameplay)),
    );

    app.run();
}
