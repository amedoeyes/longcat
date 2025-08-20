mod main;
mod navigate;
mod over;
mod pause;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();
    app.add_plugins((navigate::plugin, main::plugin, pause::plugin, over::plugin));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    None,
    Main,
    Pause,
    Over,
}
