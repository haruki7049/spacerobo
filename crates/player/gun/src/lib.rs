mod gun;

use bevy::prelude::*;
use spacerobo_commons::GameMode;

pub struct GunPlugin {
    pub is_bot: bool,
}
pub use gun::{Gun, Interval, Muzzle, Ownable, Owner, bullet, gun_cooling_system, select_fire};

impl std::default::Default for GunPlugin {
    fn default() -> Self {
        Self { is_bot: false }
    }
}

impl GunPlugin {
    pub fn is_bot(&self) -> bool {
        self.is_bot
    }
}

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        if self.is_bot {
            app.add_systems(
                Update,
                (gun::select_fire::full_auto_forever_system).run_if(in_state(GameMode::InGame)),
            );
        } else {
            app.add_systems(
                Update,
                (
                    gun::select_fire::full_auto_system,
                    gun::select_fire::semi_auto_system,
                    gun::select_fire::toggle_select_fire_system,
                )
                    .run_if(in_state(GameMode::InGame)),
            );
        }

        app.add_systems(
            FixedUpdate,
            (gun::gun_cooling_system).run_if(in_state(GameMode::InGame)),
        );
    }
}
