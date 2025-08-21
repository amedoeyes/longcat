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
    mut buttons: Query<(Entity, &mut BackgroundColor, &Children), With<Button>>,
    mut text: Query<&mut TextColor>,
    input_focus: Res<InputFocus>,
) {
    for (entity, mut background_color, children) in buttons.iter_mut() {
        let mut text_color = text.get_mut(children[0]).unwrap();
        if input_focus.0 == Some(entity) {
            background_color.0 = Color::srgb_u8(0xa0, 0xa0, 0xa0);
            text_color.0 = Color::srgb_u8(0x00, 0x00, 0x00);
        } else {
            background_color.0 = Color::NONE;
            text_color.0 = Color::srgb_u8(0xa0, 0xa0, 0xa0);
        }
    }
}
