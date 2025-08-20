use bevy::{
    input::common_conditions::input_just_pressed,
    input_focus::{
        InputDispatchPlugin, InputFocus, directional_navigation::DirectionalNavigation,
        directional_navigation::DirectionalNavigationPlugin,
    },
    math::CompassOctant,
    prelude::*,
};

use crate::menus::Menu;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin));
    app.add_systems(
        Update,
        (
            (
                navigate_up.run_if(input_just_pressed(KeyCode::ArrowUp)),
                navigate_down.run_if(input_just_pressed(KeyCode::ArrowDown)),
            ),
            highlight_focused.run_if(resource_changed::<InputFocus>),
        )
            .chain()
            .run_if(not(in_state(Menu::None))),
    );
}

fn navigate_up(mut dir_nav: DirectionalNavigation) {
    dir_nav.navigate(CompassOctant::North).unwrap();
}

fn navigate_down(mut dir_nav: DirectionalNavigation) {
    dir_nav.navigate(CompassOctant::South).unwrap();
}

fn highlight_focused(
    mut query: Query<(Entity, &mut TextColor), With<Button>>,
    input_focus: Res<InputFocus>,
) {
    for (entity, mut text_color) in query.iter_mut() {
        if input_focus.0 == Some(entity) {
            text_color.0 = Color::srgb(1.0, 0.0, 0.0);
        } else {
            text_color.0 = Color::WHITE;
        }
    }
}
