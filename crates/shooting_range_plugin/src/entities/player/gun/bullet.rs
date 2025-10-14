use bevy::prelude::*;

pub mod health;

/// A marker component for a bullet shot by a Gun
#[derive(Component)]
pub struct Bullet;
