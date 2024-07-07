use crate::character::MovementAction;
use crate::player::Player;
use bevy::prelude::*;
use game_camera::CameraFocus;

pub(crate) fn player_keyboard_input_system(
    player_query: Query<Entity, With<Player>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(entity) = player_query.get_single() else {
        return;
    };

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
        movement_event_writer.send(MovementAction::Move { direction, entity });
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump { entity });
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
