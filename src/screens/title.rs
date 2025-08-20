use bevy::prelude::*;

use crate::{
    game::{GameState, InGame},
    menus::Menu,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), open_main_menu);
    app.add_systems(OnExit(Screen::Title), close_main_menu);
}

fn open_main_menu(
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_in_game: ResMut<NextState<InGame>>,
) {
    next_menu.set(Menu::Main);
    next_game_state.set(GameState::None);
    next_in_game.set(InGame::False);
}

fn close_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
