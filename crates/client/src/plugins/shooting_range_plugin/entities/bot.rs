//! # Bot systems, Compoments & etc...

pub mod gun;
pub mod health;

use bevy::prelude::*;
use crate::plugins::commons::{DeathEvent, GameMode};

/// Bot Component
#[derive(Component)]
pub struct Bot;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>();
        app.add_systems(
            Update,
            (
                // Gun
                gun::select_fire::full_auto_system,
                gun::bullet::health::update_system,
                // health system
                health::update_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        );
        app.add_systems(
            FixedUpdate,
            (
                // gun systems
                gun::gun_cooling_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        );
    }
}
