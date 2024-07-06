use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct PrimaryCamera;

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct CameraFocus;

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct CameraSettings {
    pub sensitivity: f32,
    pub zoom_bounds: (f32, f32),
}

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct CameraZoom(pub f32);

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            zoom_bounds: (5.0, 25.0),
        }
    }
}

impl Default for CameraZoom {
    fn default() -> Self {
        Self(5.0)
    }
}
