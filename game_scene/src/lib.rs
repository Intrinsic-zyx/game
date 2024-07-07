use crate::atmosphere::AtmospherePlugin;
use bevy::app::PluginGroupBuilder;
use bevy::core_pipeline::auto_exposure::AutoExposurePlugin;
use bevy::prelude::*;

mod atmosphere;

pub struct ScenePlugins;

impl PluginGroup for ScenePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(AutoExposurePlugin).add(AtmospherePlugin);

        group
    }
}
