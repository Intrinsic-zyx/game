mod components;
mod systems;

use crate::player::systems::{
    focus_on_player_system, player_gamepad_input_system, player_keyboard_input_system,
};
use bevy::prelude::*;
pub use components::*;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();

        app.add_systems(
            Update,
            (
                player_keyboard_input_system,
                player_gamepad_input_system,
                focus_on_player_system,
            ),
        );
    }
}
