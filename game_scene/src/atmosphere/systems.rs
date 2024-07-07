use bevy::core_pipeline::auto_exposure::AutoExposureSettings;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::pbr::{VolumetricFogSettings, VolumetricLight};
use bevy::prelude::*;
use game_camera::PrimaryCamera;

pub(crate) fn add_auto_exposure_system(
    camera_query: Query<Entity, Added<PrimaryCamera>>,
    mut commands: Commands,
) {
    for entity in &camera_query {
        commands
            .entity(entity)
            .insert(AutoExposureSettings::default());
    }
}

pub(crate) fn add_volumetric_fog_system(
    camera_query: Query<Entity, Added<PrimaryCamera>>,
    mut commands: Commands,
) {
    for entity in &camera_query {
        commands
            .entity(entity)
            .insert(VolumetricFogSettings::default());
    }
}

pub(crate) fn add_bloom_system(
    camera_query: Query<Entity, Added<PrimaryCamera>>,
    mut commands: Commands,
) {
    for entity in &camera_query {
        commands.entity(entity).insert(BloomSettings::default());
    }
}

pub(crate) fn add_sun_system(mut commands: Commands) {
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(100.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ShowLightGizmo::default(),
        VolumetricLight::default(),
    ));
}
