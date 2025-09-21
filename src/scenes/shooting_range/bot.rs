//! # Bot systems, Compoments & etc...

#![allow(clippy::type_complexity)]

pub mod gun;
pub mod health;

use bevy::prelude::*;

/// Bot Component
#[derive(Component)]
pub struct Bot;
