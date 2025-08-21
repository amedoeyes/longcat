use bevy::prelude::*;

pub fn button(label: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::NONE),
        children![(
            Text::new(label),
            TextFont::from_font_size(40.0),
            TextColor(Color::srgb_u8(0xa0, 0xa0, 0xa0)),
        )],
    )
}
