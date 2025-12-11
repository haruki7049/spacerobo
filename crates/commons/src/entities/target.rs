//! # Target systems, Compoments & etc...

mod health;

use bevy::prelude::*;
use crate::{DeathEvent, GameMode};

/// Target Component
#[derive(Component)]
pub struct Target;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>();
        app.add_systems(
            Update,
            (
                // Target
                health::update_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        );
    }
}
