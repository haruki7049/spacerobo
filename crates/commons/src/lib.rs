//! Spacerobo commons

use avian3d::prelude::*;
use bevy::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod configs;

#[derive(Debug, Event)]
pub struct DeathEvent {
    pub entity: Entity,
    pub sound: Option<Handle<AudioSource>>,
}

impl DeathEvent {
    pub fn new(entity: Entity, sound: Option<Handle<AudioSource>>) -> Self {
        Self { entity, sound }
    }
}

#[derive(Debug, States, Default, Hash, Eq, PartialEq, Clone)]
#[states(scoped_entities)]
pub enum GameMode {
    #[default]
    Title,
    InGame,
}

#[derive(Debug, Resource, Default, Deref)]
pub struct KillCounter {
    inner: usize,
}

impl KillCounter {
    pub fn reset(&mut self) {
        self.inner = 0;
    }

    pub fn increment(&mut self) {
        self.inner += 1;
    }

    pub fn decrement(&mut self) {
        self.inner = self.inner.saturating_sub(1);
    }
}

#[derive(Debug, Component)]
pub struct Hp {
    pub rest: f32,
    pub maximum: f32,
    pub death_sound: Option<Handle<AudioSource>>,
}

impl std::default::Default for Hp {
    fn default() -> Self {
        Self {
            rest: 100.,
            maximum: 100.,
            death_sound: None,
        }
    }
}

impl Hp {
    pub fn decrease(&mut self, v: f32) {
        self.rest -= v;
    }

    pub fn new(hp: f32, death_sound: Option<Handle<AudioSource>>) -> Self {
        Self {
            rest: hp,
            maximum: hp,
            death_sound,
        }
    }

    pub fn ammo() -> Self {
        let hp: f32 = 5.;
        let death_sound = None;

        Self {
            rest: hp,
            maximum: hp,
            death_sound,
        }
    }

    pub fn robo(death_sound: Option<Handle<AudioSource>>) -> Self {
        let hp = 100.;

        Self {
            rest: hp,
            maximum: hp,
            death_sound,
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
            let opponent_resource: OpponentResource = OpponentResource::default();
            assert!(opponent_resource.get().is_none());
        }

        /// set method's unit test
        #[test]
        fn set() {
            let info: Information = Information {
                player: None,
                bullets: Vec::new(),
            };
            let mut opponent_resource: OpponentResource = OpponentResource::default();
            opponent_resource.set(info);

            assert!(opponent_resource.get().is_some());
        }

        /// reset method's unit test
        #[test]
        fn reset() {
            let info: Information = Information {
                player: None,
                bullets: Vec::new(),
            };
            let mut opponent_resource: OpponentResource = OpponentResource::default();
            opponent_resource.set(info);
            opponent_resource.reset();

            assert!(opponent_resource.get().is_none());
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
            let event: DeathEvent = DeathEvent::new(entity, None);
            assert_eq!(event.entity, entity);
        }
    }

    /// KillCounter's unit tests
    mod kill_counter {
        use crate::KillCounter;

        /// A test to check Default trait's implementation for KillCounter
        #[test]
        fn default() {
            let default: KillCounter = KillCounter::default();
            assert_eq!(default.inner, 0);
        }

        /// A test to check Deref trait's implementation for KillCounter
        #[test]
        fn deref() {
            let counter: KillCounter = KillCounter::default();
            assert_eq!(*counter, 0);
        }

        /// increment method's unit test
        #[test]
        fn increment() {
            let mut counter: KillCounter = KillCounter::default();
            counter.increment();
            assert_eq!(*counter, 1);
        }

        /// decrement method's unit test
        #[test]
        fn decrement() {
            let mut counter: KillCounter = KillCounter::default();

            // Three times imcrementing
            counter.increment();
            counter.increment();
            counter.increment();

            // A decrementing
            counter.decrement();
            assert_eq!(*counter, 2);
        }

        // This test is commented out because it relied on the panic behavior of standard subtraction (usize - 1)
        // which changes between debug (panic) and release (wrap) profiles.
        // The main `decrement` logic has been changed to use `saturating_sub` (safe, saturates at 0)
        // for better robustness in a release environment, making the panic check obsolete.
        /*
        /// decrement method's unit test when the inner value is overflow
        #[test]
        #[should_panic(expected = "attempt to subtract with overflow")]
        fn decrement_overflow() {
            let mut counter: KillCounter = KillCounter::default();
            counter.decrement();
        }
        */

        /// decrement method's unit test when the inner value tries to underflow (saturates at 0)
        #[test]
        fn decrement_saturating() {
            let mut counter: KillCounter = KillCounter::default();
            counter.decrement();
            // Should saturate at 0, not wrap around or panic.
            assert_eq!(*counter, 0);
        }
    }
}
