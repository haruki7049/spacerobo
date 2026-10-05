//! Spacerobo commons

use bevy::prelude::*;
use configs::GameConfigs;

pub use spacerobo_configs as configs;

mod controllable;

pub use controllable::{Controllable, ControllablePlugin};

#[derive(Debug, Message)]
pub struct DeathMessage {
    pub entity: Entity,
}

impl DeathMessage {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Debug, Event)]
pub struct Damage {
    pub target: Entity,
    pub amount: f32,
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
        self.inner = self.inner.saturating_add(1);
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

/// Rounds remaining in a gun's magazine, and the magazine's capacity.
///
/// Lives on the gun's owner (the same entity `Hp` lives on), so both `spacerobo_gun` (which
/// consumes and reloads it) and `spacerobo_hud` (which displays it) can depend on it without
/// depending on each other.
#[derive(Debug, Component)]
pub struct Ammo {
    pub rest: u32,
    pub capacity: u32,
}

impl Ammo {
    /// Creates a full magazine of `capacity` rounds.
    pub fn new(capacity: u32) -> Self {
        Self {
            rest: capacity,
            capacity,
        }
    }

    /// Spends one round. Returns `false` without changing `rest` if the magazine is empty.
    pub fn consume(&mut self) -> bool {
        if self.rest == 0 {
            return false;
        }

        self.rest = self.rest.saturating_sub(1);
        true
    }

    /// Refills the magazine to its capacity.
    pub fn reload(&mut self) {
        self.rest = self.capacity;
    }
}

pub trait Bullet {
    fn shoot(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        origin: Vec3,
        force: Vec3,
        owner: Entity,
    );

    fn gunfire_sound(commands: &mut Commands, asset_server: &Res<AssetServer>, place: Vec3);
    fn owner(&self) -> Entity;
    fn bounce_count(&self) -> usize;
}

pub trait Player {
    fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        kill_counter: &mut ResMut<KillCounter>,
        asset_server: Res<AssetServer>,
        game_configs: &GameConfigs,
    );
}

pub trait Target {
    fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        base_color: Color,
        position: Vec3,
    );
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

#[cfg(test)]
mod tests {
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

    /// DeathMessage's unit tests
    mod death_message {
        use crate::DeathMessage;
        use bevy::prelude::*;

        /// new method's unit test
        #[test]
        fn new() {
            let entity: Entity = Entity::PLACEHOLDER; // A placeholder value
            let event: DeathMessage = DeathMessage::new(entity);
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

        /// increment saturates instead of overflowing
        #[test]
        fn increment_saturates_at_max() {
            let mut counter: KillCounter = KillCounter { inner: usize::MAX };
            counter.increment();
            assert_eq!(*counter, usize::MAX);
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

    /// Ammo's unit tests
    mod ammo {
        use crate::Ammo;

        /// `new` starts with a full magazine.
        #[test]
        fn new_starts_full() {
            let ammo = Ammo::new(8);
            assert_eq!(ammo.rest, 8);
            assert_eq!(ammo.capacity, 8);
        }

        /// `consume` spends one round and reports success.
        #[test]
        fn consume_spends_one_round() {
            let mut ammo = Ammo::new(2);
            assert!(ammo.consume());
            assert_eq!(ammo.rest, 1);
        }

        /// `consume` on an empty magazine changes nothing and reports failure.
        #[test]
        fn consume_fails_when_empty() {
            let mut ammo = Ammo::new(0);
            assert!(!ammo.consume());
            assert_eq!(ammo.rest, 0);
        }

        /// `reload` refills the magazine back to capacity.
        #[test]
        fn reload_refills_to_capacity() {
            let mut ammo = Ammo::new(8);
            ammo.consume();
            ammo.consume();
            ammo.reload();
            assert_eq!(ammo.rest, 8);
        }
    }
}
