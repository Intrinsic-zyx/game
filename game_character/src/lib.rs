use crate::character::CharacterPlugin;
use crate::player::PlayerPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub use character::*;
pub use player::*;

mod character;
mod player;

pub struct CharacterPlugins;

impl PluginGroup for CharacterPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add(CharacterPlugin).add(PlayerPlugin);

        builder
    }
}
