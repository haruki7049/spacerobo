use super::PlayerInfo;
use bevy::prelude::*;
use bevy_octopus::prelude::*;

pub fn update_system(
    //mut commands: Commands,
    mut channel_received: EventReader<ReceiveChannelMessage<PlayerInfo>>,
) {
    for event in channel_received.read() {
        info!("Received: {:?}", event);
    }
}
