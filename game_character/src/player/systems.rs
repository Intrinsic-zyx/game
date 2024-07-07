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
        let x = right as i8 - left as i8;
        let y = down as i8 - up as i8;
        let direction = Vec3::new(x as f32, 0.0, y as f32);
        direction.normalize_or_zero()
    };

    if direction != Vec3::ZERO {
        movement_event_writer.send(MovementAction::Move { direction, entity });
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump { entity });
    }
}

pub(crate) fn player_gamepad_input_system(
    player_query: Query<Entity, With<Player>>,
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
) {
    let Ok(entity) = player_query.get_single() else {
        return;
    };

    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY,
        };

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            let direction = Vec3::new(x, 0.0, y).normalize_or_zero();
            movement_event_writer.send(MovementAction::Move { direction, entity });
        }

        let jump_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };

        if buttons.just_pressed(jump_button) {
            movement_event_writer.send(MovementAction::Jump { entity });
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
