use avian3d::PhysicsPlugins;
use bevy::app::PluginGroupBuilder;
use bevy::core_pipeline::auto_exposure::AutoExposurePlugin;
use bevy::prelude::*;
use game_camera::CameraPlugin;
use game_character::CharacterPlugin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add_group(PhysicsPlugins::default())
            .add(AutoExposurePlugin)
            .add(CameraPlugin)
            .add(CharacterPlugin);

        group
    }
}
