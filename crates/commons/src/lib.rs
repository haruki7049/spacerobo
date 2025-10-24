//! Spacerobo commons

use avian3d::prelude::*;
use bevy::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod configs;

#[derive(Debug, Event, Deref)]
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
    pub fn decrease(&mut self, v: f32) {
        self.rest -= v;
    }

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
    pub player: Option<PlayerInformation>,
    pub bullets: Vec<BulletInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInformation {
    pub transform: Transform,
    pub angular: AngularVelocity,
    pub linear: LinearVelocity,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulletInformation {
    pub transform: Transform,
    pub angular: AngularVelocity,
    pub linear: LinearVelocity,
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

#[cfg(test)]
mod tests {
    /// OpponentResource's unit tests
    mod opponent_resource {
        use crate::{Information, OpponentResource};
        use bevy::prelude::*;

        /// get method's unit test
        #[test]
        fn get() {
            let mut app = App::new();

            let opponent_resource: OpponentResource = OpponentResource::default();
            app.insert_resource(opponent_resource);

            assert!(app.world().resource::<OpponentResource>().get().is_none());
        }

        /// set method's unit test
        #[test]
        fn set() {
            let mut app = App::new();

            let info: Information = Information {
                player: None,
                bullets: Vec::new(),
            };
            let mut opponent_resource: OpponentResource = OpponentResource::default();
            opponent_resource.set(info);
            app.insert_resource(opponent_resource);

            assert!(app.world().resource::<OpponentResource>().get().is_some());
        }

        /// reset method's unit test
        #[test]
        fn reset() {
            let mut app = App::new();

            let info: Information = Information {
                player: None,
                bullets: Vec::new(),
            };
            let mut opponent_resource: OpponentResource = OpponentResource::default();
            opponent_resource.set(info);
            opponent_resource.reset();
            app.insert_resource(opponent_resource);

            assert!(app.world().resource::<OpponentResource>().get().is_none());
        }
    }

    /// GameMode's unit tests
    mod game_mode {
        use crate::GameMode;

        /// A test to check Default trait's implementation for GameMode
        #[test]
        fn default() {
            let default: GameMode = GameMode::default();
            assert_eq!(default, GameMode::Title);
        }
    }

    /// DeathEvent's unit tests
    mod death_event {
        use crate::DeathEvent;
        use bevy::prelude::*;

        /// new method's unit test
        #[test]
        fn new() {
            let entity: Entity = Entity::PLACEHOLDER; // A placeholder value
            let event: DeathEvent = DeathEvent::new(entity);
            assert_eq!(event.entity, entity);
        }

        /// A test to check Deref trait's implementation for DeathEvent
        #[test]
        fn deref() {
            let entity: Entity = Entity::PLACEHOLDER; // A placeholder value
            let event: DeathEvent = DeathEvent {
                entity: entity,
            };
            assert_eq!(*event, entity);
        }
    }
}
