mod gun;

use bevy::prelude::*;
use spacerobo_commons::GameMode;

#[derive(Default)]
pub struct GunPlugin;
pub use gun::{Gun, Interval, Muzzle, bullet, gun_cooling_system, select_fire};

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                gun::select_fire::full_auto_system,
                gun::select_fire::semi_auto_system,
                gun::select_fire::toggle_select_fire_system,
            )
                .run_if(in_state(GameMode::InGame)),
        );

        app.add_systems(
            FixedUpdate,
            (gun::gun_cooling_system).run_if(in_state(GameMode::InGame)),
        );
    }
}
