use crate::{CameraFocus, CameraSettings, CameraZoom, PrimaryCamera};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub(crate) fn add_missing_components_system(
    camera_query: Query<(Entity, &CameraSettings), Added<CameraSettings>>,
    mut commands: Commands,
) {
    for (entity, camera_settings) in &camera_query {
        let entity_commands = commands.get_entity(entity);
        match entity_commands {
            Some(mut commands) => {
                let (min, max) = camera_settings.zoom_bounds;
                let zoom = ((min + max) / 2.0).clamp(min, max);
                commands.insert(CameraZoom(zoom));
            }
            None => (),
        }
    }
}

pub(crate) fn scroll_zooms_camera_system(
    mut camera_query: Query<(&mut CameraZoom, &CameraSettings), With<PrimaryCamera>>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
) {
    let Ok((mut camera_zoom, camera_settings)) = camera_query.get_single_mut() else {
        return;
    };
    let zoom = camera_zoom.0;
    let (min, max) = camera_settings.zoom_bounds;
    let delta_zoom: f32 = mouse_wheel_reader.read().map(|event| event.y).sum();
    camera_zoom.0 = (zoom - delta_zoom).clamp(min, max);
}

pub(crate) fn mouse_moves_camera_system(
    mut camera_query: Query<(&mut Transform, &CameraSettings, &CameraZoom), With<PrimaryCamera>>,
    focus_query: Query<&Transform, (With<CameraFocus>, Without<PrimaryCamera>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut mouse_motion_reader: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    let Ok((mut camera_transform, camera_settings, camera_zoom)) = camera_query.get_single_mut()
    else {
        return;
    };
    let Ok(camera_focus) = focus_query.get_single() else {
        return;
    };
    let Ok(primary_window) = window_query.get_single() else {
        return;
    };
    let delta_time = time.delta_seconds();
    let (yaw, pitch): (f32, f32) = {
        let window_scale = primary_window.height().min(primary_window.width());
        let delta = match mouse_button_input.pressed(MouseButton::Right) {
            true => mouse_motion_reader.read().map(|event| event.delta).sum(),
            false => Vec2::ZERO,
        } * window_scale
            * camera_settings.sensitivity;
        let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
        yaw -= delta.x.to_radians();
        pitch = (pitch - delta.y.to_radians()).clamp(-1.54, 1.54);
        (yaw, pitch)
    };
    let rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    let translation: Vec3 = {
        let matrix = Mat3::from_quat(rotation);
        let translation = matrix.mul_vec3(Vec3::Z * camera_zoom.0) + camera_focus.translation;
        translation
    };
    camera_transform.rotation = rotation;
    camera_transform.translation = translation;
}
