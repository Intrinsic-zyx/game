use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Game".into(),
            present_mode: PresentMode::FifoRelaxed,
            ..default()
        }),
        ..default()
    }).set(LogPlugin {
        filter: "wgpu=off,game=debug".into(),
        ..default()
    }));

    app.run();
}
