mod plugin;
mod setup;

use crate::plugin::GamePlugins;
use crate::setup::setup;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game".into(),
                    present_mode: PresentMode::FifoRelaxed,
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                filter: "wgpu=off,game=debug,game_camera=debug,game_character=debug,game_camera=debug,game_debug=debug".into(),
                ..default()
            }),
    )
    .add_plugins(GamePlugins);

    app.add_systems(Startup, setup);

    app.run();
}
