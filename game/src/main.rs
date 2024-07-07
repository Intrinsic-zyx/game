mod plugin;
mod setup;

use crate::plugin::GamePlugins;
use crate::setup::setup;
use bevy::color::palettes::css::SKY_BLUE;
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
                filter: "wgpu=off,game=debug".into(),
                ..default()
            }),
    )
    .add_plugins(GamePlugins);

    app.insert_resource(ClearColor(Color::from(SKY_BLUE)));

    app.add_systems(Startup, setup);

    app.run();
}
