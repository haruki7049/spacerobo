//! Spacerobo

use avian3d::prelude::*;
use bevy::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    VersusMaster,
    VersusGuest,
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
    maximum: f32,
}

impl std::default::Default for Hp {
    fn default() -> Self {
        Self {
            rest: 100.,
            maximum: 100.,
        }
    }
}

impl Hp {
    pub fn rest(&self) -> f32 {
        self.rest
    }

    pub fn maximum(&self) -> f32 {
        self.maximum
    }

    pub fn new(hp: f32) -> Self {
        Self {
            rest: hp,
            maximum: hp,
        }
    }

    pub fn ammo() -> Self {
        let hp: f32 = 5.;

        Self {
            rest: hp,
            maximum: hp,
        }
    }

    pub fn player() -> Self {
        let hp: f32 = 100.;

        Self {
            rest: hp,
            maximum: hp,
        }
    }

    pub fn target() -> Self {
        let hp: f32 = 100.;

        Self {
            rest: hp,
            maximum: hp,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Information {
    pub player: PlayerInformation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInformation {
    pub transform: Transform,
    pub angular: AngularVelocity,
    pub linear: LinearVelocity,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Resource, Default)]
pub struct OpponentResource {
    pub inner: Option<Information>,
}

impl OpponentResource {
    pub fn get(&self) -> Option<Information> {
        self.inner.clone()
    }

    pub fn set(&mut self, info: Information) {
        self.inner = Some(info);
    }

    pub fn reset(&mut self) {
        self.inner = None;
    }
}
