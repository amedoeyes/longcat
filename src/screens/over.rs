use bevy::prelude::*;

use crate::{game::GameState, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Over), open_over_menu);
    app.add_systems(OnExit(Screen::Over), close_over_menu);
}

fn open_over_menu(
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_gameplay: ResMut<NextState<GameState>>,
) {
    next_menu.set(Menu::Over);
    next_gameplay.set(GameState::Over);
}

fn close_over_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
