use bevy::{prelude::*, window::WindowResized};

use crate::game::{GameState, GameSystems};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CellSize>();

    app.add_systems(
        OnExit(GameState::None),
        ((setup_cell_size, move_camera_to_grid)
            .chain()
            .in_set(GameSystems::Resources),),
    );

    app.add_systems(
        Update,
        (
            handle_window_resize,
            (resize_cells, move_camera_to_grid).run_if(resource_changed::<CellSize>),
        )
            .chain()
            .in_set(GameSystems::Update),
    );
}

pub const GRID_CELLS: IVec2 = IVec2::new(20, 10);
pub const GRID_CENTER: IVec2 = IVec2::new(GRID_CELLS.x / 2, GRID_CELLS.y / 2);

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct CellSize(f32);

#[derive(Component, Clone, Copy, PartialEq, Deref, DerefMut, Debug)]
pub struct Cell(pub Vec2);

pub fn cell(position: Vec2, size: f32) -> impl Bundle {
    (
        Cell(position),
        Transform::from_translation(Vec3::from((position * size, 2.0)))
            .with_scale(Vec3::splat(size)),
    )
}

fn move_camera_to_grid(mut camera: Single<&mut Transform, With<Camera2d>>, size: Res<CellSize>) {
    camera.translation = Vec3::from(((GRID_CELLS.as_vec2() * **size / 2.0) - **size / 2.0, 0.0));
}

fn setup_cell_size(window: Single<&Window>, mut size: ResMut<CellSize>) {
    **size = (Vec2::new(window.width(), window.height()) / GRID_CELLS.as_vec2()).min_element();
}

fn handle_window_resize(mut resize_reader: EventReader<WindowResized>, mut size: ResMut<CellSize>) {
    if let Some(e) = resize_reader.read().last() {
        **size = (Vec2::new(e.width, e.height) / GRID_CELLS.as_vec2()).min_element();
    }
}

fn resize_cells(mut cells: Query<(&mut Transform, &Cell)>, size: Res<CellSize>) {
    for (mut transform, cell) in &mut cells {
        transform.translation = Vec3::from((**cell * **size, transform.translation.z));
        transform.scale = Vec3::splat(**size);
    }
}
