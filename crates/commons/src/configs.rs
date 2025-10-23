use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod network;
pub mod player;

/// Includes player configuration
#[derive(Resource, Serialize, Deserialize, Debug, Default, Clone)]
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
