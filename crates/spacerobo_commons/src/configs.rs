use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod network;
pub mod player;

/// Includes player configuration
#[derive(Resource, Serialize, Deserialize, Debug, Default)]
pub struct GameConfigs {
    pub player: player::Config,
    pub network: network::Config,
}
