use std::collections::VecDeque;

use bevy::prelude::*;

use crate::game::{
    GameSystems,
    cat::{CatBody, CatHead, CatTail, Direction},
    grid::{Cell, GRID_CELLS},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<InputBuffer>();
    app.add_systems(
        Update,
        (
            control_cat.in_set(GameSystems::Input),
            update_cat_head_dir.in_set(GameSystems::PreFixedUpdate),
        ),
    );
}

#[derive(Resource, Default)]
pub struct InputBuffer {
    deque: VecDeque<Direction>,
}

impl InputBuffer {
    pub fn push(&mut self, item: Direction) {
        if self.deque.len() == 4 {
            self.deque.pop_front();
        }
        self.deque.push_back(item);
    }

    pub fn pop(&mut self) -> Option<Direction> {
        self.deque.pop_front()
    }
}

fn control_cat(keyboard_input: Res<ButtonInput<KeyCode>>, mut input_buffer: ResMut<InputBuffer>) {
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        input_buffer.push(Direction::Left);
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        input_buffer.push(Direction::Right);
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        input_buffer.push(Direction::Up);
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        input_buffer.push(Direction::Down);
    }
}

fn update_cat_head_dir(
    head: Single<(&Cell, &mut Direction), With<CatHead>>,
    rest: Query<(Entity, &Cell), (Or<(With<CatBody>, With<CatTail>)>, Without<CatHead>)>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    let (cell, mut dir) = head.into_inner();
    while let Some(new_dir) = input_buffer.pop() {
        if new_dir == *dir || -new_dir.to_vec() == dir.to_vec() {
            continue;
        }

        let next_cell = (**cell + new_dir.to_vec()).rem_euclid(GRID_CELLS.as_vec2());
        if rest.iter().any(|(_, cell)| **cell == next_cell) {
            continue;
        }

        *dir = new_dir;
        break;
    }
}
