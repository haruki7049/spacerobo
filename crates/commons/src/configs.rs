use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod player;

/// Includes player configuration
#[derive(Resource, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct GameConfigs {
    player: player::Config,
}

impl GameConfigs {
    pub fn player(&self) -> player::Config {
        self.player.clone()
    }
}

#[cfg(test)]
mod tests {
    mod game_configs {
        use crate::configs::{GameConfigs, player};

        #[test]
        fn player() {
            let configs: GameConfigs = GameConfigs::default();
            let player_default_configs: player::Config = player::Config::default();

            assert_eq!(configs.player(), player_default_configs);
        }
    }
}
