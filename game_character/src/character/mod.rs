mod components;
mod systems;

use crate::character::systems::{
    apply_character_movement_dampening_system, character_movement_system, spawn_characters_system,
    update_character_grounded_system,
};
use bevy::prelude::*;
pub use components::*;

pub(crate) struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Character>()
            .register_type::<Grounded>()
            .register_type::<MovementSpeed>()
            .register_type::<MovementDamping>()
            .register_type::<JumpImpulse>()
            .register_type::<MaxSlopeAngle>();

        app.add_event::<MovementAction>()
            .add_event::<SpawnCharacter>();

        app.add_systems(
            Update,
            (
                spawn_characters_system,
                character_movement_system,
                apply_character_movement_dampening_system,
                update_character_grounded_system,
            )
                .chain(),
        );
    }
}
