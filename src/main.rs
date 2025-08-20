#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod game;
mod menus;
mod screens;
mod ui;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins((menus::plugin, screens::plugin, game::plugin));
    app.insert_resource(ClearColor(Color::BLACK));
    app.add_systems(Startup, setup_camera);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
