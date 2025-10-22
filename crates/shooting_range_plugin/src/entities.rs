pub mod bot;
pub mod player;
pub mod target;

use bevy::prelude::*;
use spacerobo_commons::{DeathEvent, GameMode, KillCounter};

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bot::BotPlugin,
            target::TargetPlugin,
        ));
        app.add_event::<DeathEvent>();
        app.insert_resource(KillCounter::default());
        app.add_systems(
            OnEnter(GameMode::ShootingRange),
            (player::setup_system, player::ui::setup_system),
        );
        app.add_systems(
            Update,
            (
                // Player
                player::respawn_system,
                player::ui::update_system,
                player::gun::select_fire::full_auto_system,
                player::gun::select_fire::semi_auto_system,
                player::gun::select_fire::toggle_select_fire_system,
                player::gun::bullet::health::update_system,
                player::health::update_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        );
        app.add_systems(
            FixedUpdate,
            (
                // Player movement systems
                player::movement::keyboard::update_system,
                player::movement::mouse::update_system,
                player::movement::controller::update_system,
                // Player gun systems
                player::gun::gun_cooling_system,
            )
                .run_if(in_state(GameMode::ShootingRange)),
        );
    }
}
