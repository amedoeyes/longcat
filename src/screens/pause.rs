use bevy::prelude::*;

use crate::{game::GameState, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Pause), open_pause_menu);
    app.add_systems(OnExit(Screen::Pause), close_pause_menu);
}

fn open_pause_menu(
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_gameplay: ResMut<NextState<GameState>>,
) {
    next_menu.set(Menu::Pause);
    next_gameplay.set(GameState::Pause);
}

fn close_pause_menu(
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_gameplay: ResMut<NextState<GameState>>,
) {
    next_menu.set(Menu::None);
    next_gameplay.set(GameState::Run);
}
