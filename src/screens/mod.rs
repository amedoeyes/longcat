mod gameplay;
mod over;
mod pause;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_plugins((title::plugin, gameplay::plugin, pause::plugin, over::plugin));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Title,
    Gameplay,
    Pause,
    Over,
}
