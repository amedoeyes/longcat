use bevy::{
    ecs::spawn::SpawnWith,
    input::keyboard::KeyboardInput,
    input_focus::{FocusedInput, InputFocus, directional_navigation::DirectionalNavigationMap},
    math::CompassOctant,
    prelude::*,
};

use crate::{menus::Menu, screens::Screen, ui::button};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
}

fn spawn_pause_menu(
    mut commands: Commands,
    mut dir_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    let entries = [
        commands
            .spawn(button("Resume"))
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
                 mut next_screen: ResMut<NextState<Screen>>| {
                    if keyboard_input.just_pressed(KeyCode::Enter) {
                        next_screen.set(Screen::Title);
                    }
                },
            )
            .id(),
    ];

    dir_nav_map.add_looping_edges(&entries, CompassOctant::South);
    input_focus.set(entries[0]);

    commands.spawn((
        StateScoped(Menu::Pause),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .add_children(&entries);
        })),
    ));
}
