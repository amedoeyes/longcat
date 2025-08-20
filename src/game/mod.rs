mod atlas;
mod cat;
mod control;
mod food;
mod grid;
mod level;
mod ticktimer;

use bevy::prelude::*;

use crate::game::ticktimer::tick_passed;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        atlas::plugin,
        ticktimer::plugin,
        grid::plugin,
        control::plugin,
        cat::plugin,
        level::plugin,
    ));

    app.init_state::<GameState>();
    app.init_state::<InGame>();

    app.configure_sets(
        OnExit(GameState::None),
        (GameSystems::Resources, GameSystems::Spawn).chain(),
    );

    app.configure_sets(
        Update,
        (
            (
                GameSystems::TickTimers,
                GameSystems::Input,
                (GameSystems::PreFixedUpdate, GameSystems::FixedUpdate)
                    .chain()
                    .run_if(tick_passed()),
            )
                .chain()
                .run_if(in_state(GameState::Run)),
            GameSystems::Update,
        )
            .chain()
            .run_if(not(in_state(GameState::None))),
    );
}

#[derive(States, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    None,
    Run,
    Pause,
    Over,
}

#[derive(States, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
#[states(scoped_entities)]
pub enum InGame {
    #[default]
    False,
    True,
}

#[derive(SystemSet, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameSystems {
    Resources,
    Spawn,
    TickTimers,
    Input,
    PreFixedUpdate,
    FixedUpdate,
    Update,
}
