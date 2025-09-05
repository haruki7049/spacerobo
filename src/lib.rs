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

#[derive(Debug, Resource, Default)]
pub struct KillCounter {
    inner: usize,
}

impl KillCounter {
    pub fn get(&self) -> usize {
        self.inner
    }

    pub fn reset(&mut self) {
        self.inner = 0;
    }

    pub fn increment(&mut self) {
        self.inner += 1;
    }

    pub fn decrement(&mut self) {
        self.inner -= 1;
    }
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
    pub fn new(hp: f32) -> Self {
        Self { rest: hp }
    }

    pub fn ammo() -> Self {
        Self { rest: 5. }
    }

    pub fn player() -> Self {
        Self { rest: 100. }
    }

    pub fn target() -> Self {
        Self { rest: 100. }
    }
}
