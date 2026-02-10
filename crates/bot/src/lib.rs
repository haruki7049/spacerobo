//! # Bot systems, Compoments & etc...

use bevy::prelude::*;
use spacerobo_bot_gun::GunPlugin;

/// Bot Component
#[derive(Component)]
pub struct Bot;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GunPlugin { is_bot: true });
    }
}
