pub mod bot;

use bevy::{app::PluginGroupBuilder, prelude::*};
use spacerobo_player::PlayerPlugin;
use spacerobo_target::TargetPlugin;

pub struct EntityPlugins;

impl PluginGroup for EntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bot::BotPlugin)
            .add(PlayerPlugin)
            .add(TargetPlugin)
    }
}
