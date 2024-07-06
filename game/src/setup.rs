use avian3d::prelude::{Collider, RigidBody};
use bevy::color::palettes::basic::SILVER;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use game_camera::{CameraFocus, CameraSettings, PrimaryCamera};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut spawn_character_writer: EventWriter<SpawnCharacter>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 7.0, 14.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..default()
        },
        CameraSettings::default(),
        BloomSettings::default(),
        PrimaryCamera,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 1_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
            material: materials.add(Color::from(SILVER)),
            transform: Transform::IDENTITY,
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 0.002, 50.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(1.0, 2.0).mesh()),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule(1.0, 2.0),
        CameraFocus,
    ));

    // spawn_character_writer.send(SpawnCharacter {
    //     player: true,
    //     position: Vec3::new(0.0, 10.0, 0.0),
    // });
}
