use avian3d::PhysicsPlugins;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use game_camera::CameraPlugin;
use game_character::CharacterPlugin;
use game_debug::DebugPlugins;
use game_scene::ScenePlugins;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add_group(PhysicsPlugins::default())
            .add_group(DebugPlugins)
            .add_group(ScenePlugins);

        group = group.add(CameraPlugin).add(CharacterPlugin);

        group
    }
}
