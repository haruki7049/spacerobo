mod gun;

use bevy::prelude::*;
use spacerobo_commons::GameMode;

pub use gun::{Gun, Interval, Muzzle, Ownable, Owner, bullet, gun_cooling_system, select_fire};

#[derive(Default)]
pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (gun::select_fire::full_auto_forever_system).run_if(in_state(GameMode::InGame)),
        );

        app.add_systems(
            FixedUpdate,
            (gun::gun_cooling_system).run_if(in_state(GameMode::InGame)),
        );
    }
}
