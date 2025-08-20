use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    game::{GameState, InGame},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), start_game);

    app.add_systems(
        Update,
        (
            pause_game.run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape))),
            unpause_game.run_if(in_state(Screen::Pause).and(input_just_pressed(KeyCode::Escape))),
        ),
    );
}

fn start_game(
    mut next_gameplay: ResMut<NextState<GameState>>,
    mut next_in_game: ResMut<NextState<InGame>>,
) {
    next_gameplay.set(GameState::Run);
    next_in_game.set(InGame::True);
}

fn pause_game(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Pause);
}

fn unpause_game(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}
