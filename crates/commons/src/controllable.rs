//! # control systems

use bevy::prelude::*;

mod keyboard;
mod mouse;

#[derive(Component)]
pub struct Controllable;
pub struct ControllablePlugin;

impl Plugin for ControllablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard::update_system);
        app.add_systems(Update, mouse::update_system);
    }
}
