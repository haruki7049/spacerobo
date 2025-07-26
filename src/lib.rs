//! Spacerobo

use bevy::prelude::*;

pub mod cli;
pub mod configs;
pub mod scenes;

#[derive(Debug, Event)]
pub struct DeathEvent {
    entity: Entity,
}

impl DeathEvent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Debug, States, Default, Hash, Eq, PartialEq, Clone)]
#[states(scoped_entities)]
pub enum GameMode {
    #[default]
    Title,
    ShootingRange,
}

#[derive(Debug, Component)]
pub struct Hp {
    rest: f32,
}

impl std::default::Default for Hp {
    fn default() -> Self {
        Self { rest: 100. }
    }
}

impl Hp {
    pub fn ammo() -> Self {
        Self { rest: 5. }
    }
}
