use crate::systems::{
    apply_character_movement_dampening_system, focus_on_player_system,
    player_character_movement_system, player_keyboard_input_system, spawn_characters_system,
    update_character_grounded_system,
};
use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<Character>()
            .register_type::<Grounded>()
            .register_type::<MovementSpeed>()
            .register_type::<MovementDamping>()
            .register_type::<JumpImpulse>()
            .register_type::<MaxSlopeAngle>();

        app.add_event::<MovementAction>()
            .add_event::<SpawnCharacter>();

        app.add_systems(Update, player_keyboard_input_system)
            .add_systems(
                Update,
                (
                    spawn_characters_system,
                    focus_on_player_system,
                    player_character_movement_system,
                    apply_character_movement_dampening_system,
                    update_character_grounded_system,
                )
                    .chain(),
            );
    }
}
