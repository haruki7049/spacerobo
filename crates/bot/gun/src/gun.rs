//! # Gun systems, components & etc...

pub mod bullet;
pub mod select_fire;

use self::select_fire::SelectFire;
use bevy::prelude::*;

/// Gun component
#[derive(Component)]
pub struct Gun {
    pub owner: Entity,

    /// Select fire setting
    pub select_fire: SelectFire,

    /// A interval settings and values
    pub interval: Interval,
}

#[derive(Component)]
pub struct Ownable;

pub trait Owner {}

impl Owner for Ownable {}

impl Gun {
    fn fullauto(&mut self) {
        self.select_fire = SelectFire::Full;
    }

    fn semiauto(&mut self) {
        self.select_fire = SelectFire::Semi;
    }
}

/// A interval settings and values
#[derive(Default)]
pub struct Interval {
    /// The upper limit of interval
    pub limit: f32,

    /// The rest of full-auto interval
    pub rest: f32,

    /// A number for rest_interval decrementing
    pub amount: f32,
}

/// A marker component to know muzzle's transform
#[derive(Component)]
pub struct Muzzle;

/// Gun cooling system.
/// It controls full auto's shoot interval.
pub fn gun_cooling_system(mut gun: Query<&mut Gun>) {
    for mut gun in gun.iter_mut() {
        gun.interval.rest -= gun.interval.amount;
    }
}
