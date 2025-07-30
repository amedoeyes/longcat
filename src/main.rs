use bevy::{
    MinimalPlugins,
    a11y::AccessibilityPlugin,
    app::{App, Startup, Update},
    asset::AssetPlugin,
    ecs::{
        component::Component,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::InputPlugin,
    render::{RenderPlugin, pipelined_rendering::PipelinedRenderingPlugin, texture::ImagePlugin},
    time::{Time, Timer, TimerMode},
    window::{Window, WindowPlugin},
    winit::{WakeUp, WinitPlugin},
};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            InputPlugin,
            WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("werecat"),
                    ..Default::default()
                }),
                ..Default::default()
            },
            AccessibilityPlugin,
            AssetPlugin::default(),
            WinitPlugin::<WakeUp>::default(),
            RenderPlugin::default(),
            ImagePlugin::default(),
            PipelinedRenderingPlugin,
        ))
        .add_systems(Startup, add_people)
        .add_systems(Update, (update_people, greet_people).chain())
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .run();
}
