use avian3d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, Reflect)]
pub enum MovementAction {
    Move { direction: Vec3, entity: Entity },
    Jump { entity: Entity },
}

#[derive(Event, Reflect)]
pub struct SpawnCharacter {
    pub position: Vec3,
    pub player: bool,
}
#[derive(Component, Serialize, Deserialize, Reflect, Default)]
pub struct Character;

#[derive(Component, Serialize, Deserialize, Reflect, Default)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct MovementSpeed(pub f32);

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct MovementDamping(pub f32);

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct JumpImpulse(pub f32);

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct MaxSlopeAngle(pub f32);

#[derive(Bundle)]
pub struct CharacterBundle {
    pub character: Character,
    pub grounded: Grounded,
    pub ground_caster: ShapeCaster,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub locked_axis: LockedAxes,
    pub movement: MovementBundle,
    pub friction: Friction,
    pub restitution: Restitution,
    pub gravity_scale: GravityScale,
}

#[derive(Bundle)]
pub struct MovementBundle {
    speed: MovementSpeed,
    damping: MovementDamping,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl CharacterBundle {
    pub fn new(collider: Collider) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vec3::ONE * 0.99, 10);

        Self {
            character: Character,
            grounded: Grounded,
            ground_caster: ShapeCaster::new(caster_shape, Vec3::ZERO, Quat::IDENTITY, Dir3::NEG_Y)
                .with_max_time_of_impact(0.2),
            collider,
            rigid_body: RigidBody::Dynamic,
            locked_axis: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
            friction: Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            restitution: Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            gravity_scale: GravityScale(3.0),
        }
    }

    pub fn with_movement(
        mut self,
        speed: f32,
        damping: f32,
        jump_impulse: f32,
        max_slope_angle: f32,
    ) -> Self {
        self.movement = MovementBundle::new(speed, damping, jump_impulse, max_slope_angle);
        self
    }
}

impl MovementBundle {
    pub const fn new(speed: f32, damping: f32, jump_impulse: f32, max_slope_angle: f32) -> Self {
        Self {
            speed: MovementSpeed(speed),
            damping: MovementDamping(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(150.0, 0.92, 15.0, 30.0_f32.to_radians())
    }
}
