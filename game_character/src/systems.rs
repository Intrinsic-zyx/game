use crate::components::{
    Character, CharacterBundle, Grounded, JumpImpulse, MaxSlopeAngle, MovementAction,
    MovementDamping, MovementSpeed, Player, SpawnCharacter,
};
use avian3d::prelude::{Collider, LinearVelocity, ShapeHits};
use bevy::prelude::*;
use game_camera::{CameraFocus, PrimaryCamera};

pub(crate) fn player_keyboard_input_system(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let direction: Vec3 = {
        let horizontal = right as i8 - left as i8;
        let vertical = down as i8 - up as i8;
        let direction = Vec3::new(horizontal as f32, 0.0, vertical as f32);
        direction.normalize_or_zero()
    };

    if direction != Vec3::ZERO {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump);
    }
}

pub(crate) fn update_character_grounded_system(
    mut commands: Commands,
    mut character_query: Query<
        (Entity, &ShapeHits, &Transform, Option<&MaxSlopeAngle>),
        With<Character>,
    >,
) {
    for (entity, hits, transform, max_slope_angle) in &mut character_query {
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (transform.rotation * -hit.normal2)
                    .angle_between(Vec3::Y)
                    .abs()
                    <= angle.0
            } else {
                true
            }
        });
        match is_grounded {
            true => {
                commands.entity(entity).insert(Grounded);
            }
            false => {
                commands.entity(entity).remove::<Grounded>();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn player_character_movement_system(
    mut movement_action_reader: EventReader<MovementAction>,
    mut movement_query: Query<
        (
            &MovementSpeed,
            &JumpImpulse,
            &mut LinearVelocity,
            Has<Grounded>,
        ),
        With<Player>,
    >,
    camera_query: Query<&Transform, With<PrimaryCamera>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };
    let Ok((movement_speed, jump_impulse, mut linear_velocity, is_grounded)) =
        movement_query.get_single_mut()
    else {
        return;
    };
    for event in movement_action_reader.read() {
        match event {
            MovementAction::Move(direction) => {
                let matrix = Mat3::from_quat(camera_transform.rotation);
                let desired = matrix.mul_vec3(*direction) * movement_speed.0 * delta_time;
                linear_velocity.x += desired.x;
                linear_velocity.z += desired.z;
            }
            MovementAction::Jump => {
                if is_grounded {
                    linear_velocity.y = jump_impulse.0;
                }
            }
        }
    }
}

pub(crate) fn apply_character_movement_dampening_system(
    mut movement_query: Query<(&MovementDamping, &mut LinearVelocity)>,
) {
    for (damping_factor, mut linear_velocity) in &mut movement_query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0
    }
}

pub(crate) fn spawn_characters_system(
    mut spawn_character_reader: EventReader<SpawnCharacter>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in spawn_character_reader.read() {
        let mesh = Capsule3d::new(1.0, 2.0).mesh();
        let collider = Collider::capsule(1.0, 2.0);
        let mut entity = commands.spawn((
            CharacterBundle::new(collider),
            PbrBundle {
                mesh: meshes.add(mesh),
                transform: Transform::from_translation(event.position),
                ..default()
            },
        ));
        entity.with_children(move |builder| {
            builder.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 10_000.0,
                    range: 10.0,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            });
        });
        if event.player {
            entity.insert(Player);
        }
    }
}

pub(crate) fn focus_on_player_system(
    player_query: Query<Entity, Added<Player>>,
    mut commands: Commands,
) {
    let Ok(entity) = player_query.get_single() else {
        return;
    };
    let mut player = commands.entity(entity);
    player.insert(CameraFocus);
}
