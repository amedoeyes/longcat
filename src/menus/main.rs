use bevy::{
    ecs::spawn::SpawnWith,
    input::keyboard::KeyboardInput,
    input_focus::{FocusedInput, InputFocus, directional_navigation::DirectionalNavigationMap},
    math::CompassOctant,
    prelude::*,
};

use crate::{menus::Menu, screens::Screen, ui::button};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(
    mut commands: Commands,
    mut dir_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
    asset_server: Res<AssetServer>,
) {
    let entries = [
        commands
            .spawn(button("Start"))
            .observe(
                |_: Trigger<FocusedInput<KeyboardInput>>,
                 keyboard_input: Res<ButtonInput<KeyCode>>,
                 mut next_screen: ResMut<NextState<Screen>>| {
                    if keyboard_input.just_pressed(KeyCode::Enter) {
                        next_screen.set(Screen::Gameplay);
                    }
                },
            )
            .id(),
        commands
            .spawn(button("Quit"))
            .observe(
                |_: Trigger<FocusedInput<KeyboardInput>>,
                 keyboard_input: Res<ButtonInput<KeyCode>>,
                 mut event_writer: EventWriter<AppExit>| {
                    if keyboard_input.just_pressed(KeyCode::Enter) {
                        event_writer.write(AppExit::Success);
                    }
                },
            )
            .id(),
    ];

    dir_nav_map.add_looping_edges(&entries, CompassOctant::South);
    input_focus.set(entries[0]);

    commands.spawn((
        StateScoped(Menu::Main),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Children::spawn((
            Spawn((
                Node {
                    bottom: Val::Percent(20.0),
                    ..default()
                },
                ImageNode {
                    image: asset_server.load("title.png"),
                    ..default()
                },
                Transform::from_scale(Vec3::splat(2.0)),
            )),
            SpawnWith(move |parent: &mut ChildSpawner| {
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .add_children(&entries);
            }),
        )),
    ));
}
