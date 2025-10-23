//! # Gun systems, components & etc...

pub mod bullet;
pub mod select_fire;

use bevy::prelude::*;

const BULLET_SIZE: f32 = 1. / 8.;

/// Gun component
#[derive(Component, Default)]
pub struct Gun {
    /// A interval settings and values
    pub interval: Interval,
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
