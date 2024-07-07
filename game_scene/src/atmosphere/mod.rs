mod systems;

use crate::atmosphere::systems::{
    add_auto_exposure_system, add_bloom_system, add_sun_system, add_volumetric_fog_system,
};
use bevy::color::palettes::css::SKY_BLUE;
use bevy::prelude::*;

pub struct AtmospherePlugin;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::from(SKY_BLUE)));

        app.add_systems(
            Update,
            (
                add_auto_exposure_system,
                add_volumetric_fog_system,
                add_bloom_system,
            ),
        )
        .add_systems(Startup, add_sun_system);
    }
}
