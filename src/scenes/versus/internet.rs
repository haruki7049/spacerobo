use crate::{TCP_CHANNEL, UDP_CHANNEL, configs::GameConfigs};
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use bevy_octopus::{
    prelude::*,
    transports::{tcp::TcpAddress, udp::UdpAddress},
};

pub const CLIENT_PORT: u16 = 10000;
pub const SERVER_PORT: u16 = 10001;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    health: f32,
}

pub fn setup_system(
    mut commands: Commands,
    game_configs: Res<GameConfigs>
) {
    let client_address: String = format!("{}:{}", game_configs.network.ip, CLIENT_PORT);
    let server_address: String = format!("{}:{}", game_configs.network.ip, SERVER_PORT);

    // tcp client
    commands.spawn((
        NetworkBundle::new(TCP_CHANNEL),
        ClientNode(TcpAddress::new(client_address)),
    ));

    // udp server
    commands.spawn((
        NetworkBundle::new(UDP_CHANNEL),
        ServerNode(UdpAddress::new(server_address)),
    ));
}

pub fn update_system(
    mut channel_recviced: EventReader<ReceiveChannelMessage<PlayerInfo>>,
    mut ev: EventWriter<SendChannelMessage<PlayerInfo>>,
) {
    for event in channel_recviced.read() {
        info!("recevice {:?}", event.message);
        if event.channel_id == UDP_CHANNEL {
            info!("{:?}", TCP_CHANNEL);
            info!("{:?}", event);

            ev.write(SendChannelMessage {
                channel_id: TCP_CHANNEL,
                message: event.message.clone(),
            });
        }
    }
}

pub fn internet_observer(trigger: Trigger<NetworkEvent>) {
    info!("trigger {:?}", trigger.event());
}
