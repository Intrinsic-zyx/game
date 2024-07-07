use avian3d::prelude::PhysicsDebugPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;

pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(PhysicsDebugPlugin::default())
            .add(FpsOverlayPlugin::default());

        group
    }
}
