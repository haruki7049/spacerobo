pub mod bot;
pub mod player;
pub mod target;

use bevy::prelude::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((bot::BotPlugin, player::PlayerPlugin, target::TargetPlugin));
    }
}
