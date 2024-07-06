use avian3d::prelude::PhysicsSet;
use bevy::prelude::*;

mod component;
mod systems;

use crate::systems::{
    add_missing_components_system, mouse_moves_camera_system, scroll_zooms_camera_system,
};
pub use component::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PrimaryCamera>()
            .register_type::<CameraFocus>()
            .register_type::<CameraSettings>()
            .register_type::<CameraZoom>();

        app.add_systems(
            Update,
            (add_missing_components_system, scroll_zooms_camera_system),
        )
        .add_systems(
            PostUpdate,
            mouse_moves_camera_system
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );
    }
}
