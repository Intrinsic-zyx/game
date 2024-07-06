mod setup;

use crate::setup::setup;
use avian3d::PhysicsPlugins;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use game_camera::CameraPlugin;

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
    .add_plugins(PhysicsPlugins::default());

    app.add_plugins(CameraPlugin);

    app.add_systems(Startup, setup);

    app.run();
}
