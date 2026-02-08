//! # Bot systems, Compoments & etc...

pub mod health;

use bevy::prelude::*;
use spacerobo_commons::{DeathEvent, GameMode};
use spacerobo_gun::GunPlugin;

/// Bot Component
#[derive(Component)]
pub struct Bot;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GunPlugin);
        app.add_event::<DeathEvent>();
        app.add_systems(
            Update,
            (
                // health system
                health::update_system,
            )
                .run_if(in_state(GameMode::InGame)),
        );
    }
}
