use bevy::prelude::*;

pub fn button(label: &str) -> impl Bundle {
    (
        Button,
        Text::new(label),
        TextColor(Color::WHITE),
        TextFont::from_font_size(40.0),
    )
}
