pub mod bot;
pub mod player;
pub mod target;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct EntityPlugins;

impl PluginGroup for EntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bot::BotPlugin)
            .add(player::PlayerPlugin)
            .add(target::TargetPlugin)
    }
}
