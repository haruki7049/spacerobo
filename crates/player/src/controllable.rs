//! # control systems

use bevy::prelude::*;
use spacerobo_commons::GameMode;

mod keyboard;
mod mouse;

#[derive(Component)]
pub struct Controllable;
pub struct ControllablePlugin;

impl Plugin for ControllablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (keyboard::update_system, mouse::update_system)
                .run_if(in_state(GameMode::InGame)),
        );
    }
}
