//! # Player's Movement controlling systems
//!
//! In Versus mode we must send PlayerInfo to another player

pub mod controller;
pub mod keyboard;
pub mod mouse;

use super::{super::internet::PlayerInfo, super::player::Player};
use crate::{Hp, SERVER_CHANNEL};
use bevy::prelude::*;
use bevy_octopus::prelude::*;

pub fn send_player_info_system(
    mut event: EventWriter<SendChannelMessage<PlayerInfo>>,
    query: Query<(&Hp, &Transform), With<Player>>,
) {
    for (hp, transform) in query.iter() {
        let player_info: PlayerInfo = PlayerInfo {
            health: hp.rest,
            transform: *transform,
        };

        event.write(SendChannelMessage {
            channel_id: SERVER_CHANNEL,
            message: player_info,
        });
    }
}
