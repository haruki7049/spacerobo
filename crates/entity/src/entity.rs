pub mod bot;
pub mod target;

use bevy::{app::PluginGroupBuilder, prelude::*};
use spacerobo_player::PlayerPlugin;

pub struct EntityPlugins;

impl PluginGroup for EntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bot::BotPlugin)
            .add(PlayerPlugin)
            .add(target::TargetPlugin)
    }
}
