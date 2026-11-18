use bevy::prelude::*;
use bevy::window::WindowPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { 
                title: "Bevy 1st".to_string(),
                resolution: (800, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
