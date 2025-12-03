use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod network;
pub mod player;

/// Includes player configuration
#[derive(Resource, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct GameConfigs {
    player: player::Config,
    network: network::Config,
}

impl GameConfigs {
    pub fn player(&self) -> player::Config {
        self.player.clone()
    }

    pub fn network(&self) -> network::Config {
        self.network.clone()
    }
}

#[cfg(test)]
mod tests {
    mod game_configs {
        use super::super::{GameConfigs, network, player};

        #[test]
        fn player() {
            let configs: GameConfigs = GameConfigs::default();
            let player_default_configs: player::Config = player::Config::default();

            assert_eq!(configs.player(), player_default_configs);
        }

        #[test]
        fn network() {
            let configs: GameConfigs = GameConfigs::default();
            let network_default_configs: network::Config = network::Config::default();

            assert_eq!(configs.network(), network_default_configs);
        }
    }
}
