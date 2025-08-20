use bevy::prelude::*;

use crate::game::{GameState, GameSystems};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TickTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

    app.add_systems(Update, advance_tick_timer.in_set(GameSystems::TickTimers));

    app.add_systems(OnEnter(GameState::Pause), pause_tick_timer);
    app.add_systems(OnExit(GameState::Pause), unpause_tick_timer);

    app.add_systems(OnEnter(GameState::Over), pause_tick_timer);
    app.add_systems(OnExit(GameState::Over), unpause_tick_timer);
}

#[derive(Resource, Deref, DerefMut)]
struct TickTimer(Timer);

pub fn tick_passed() -> impl Condition<()> {
    IntoSystem::into_system(|timer: Res<TickTimer>| timer.just_finished())
}

fn advance_tick_timer(time: Res<Time>, mut timer: ResMut<TickTimer>) {
    timer.tick(time.delta());
}

fn pause_tick_timer(mut timer: ResMut<TickTimer>) {
    timer.reset();
    timer.pause();
}

fn unpause_tick_timer(mut timer: ResMut<TickTimer>) {
    timer.unpause();
}
