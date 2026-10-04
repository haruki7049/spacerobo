//! Player-tunable game configuration.
//!
//! This crate depends only on `bevy_ecs` (for the `Resource` derive) and `bevy_input` (for
//! `KeyCode` key bindings), not the full `bevy` crate, so config changes compile without pulling
//! in rendering, windowing or audio.

use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

pub mod player;

/// Includes player configuration
#[derive(Resource, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(default)]
pub struct GameConfigs {
    pub player: player::Config,
}

#[cfg(test)]
mod tests {
    mod game_configs {
        use crate::{GameConfigs, player};

        #[test]
        fn player() {
            let configs: GameConfigs = GameConfigs::default();
            let player_default_configs: player::Config = player::Config::default();

            assert_eq!(configs.player, player_default_configs);
        }
    }
}
