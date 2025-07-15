//! # Gun systems, components & etc...

pub mod select_fire;

use bevy::prelude::*;
use crate::player::gun::select_fire::SelectFire;

const BULLET_SIZE: f32 = 1. / 16.;

/// Gun component
#[derive(Component, Default)]
pub struct Gun {
    /// Select fire setting
    pub select_fire: SelectFire,

    /// A interval settings and values
    pub interval: Interval,
}

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

/// A marker component for a bullet shot by a Gun
#[derive(Component)]
pub struct Bullet;

/// Gun cooling system.
/// It controls full auto's shoot interval.
pub fn gun_cooling_system(mut gun: Query<&mut Gun>) {
    for mut gun in gun.iter_mut() {
        gun.interval.rest -= gun.interval.amount;
    }
}
