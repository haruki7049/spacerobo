use crate::{CLIENT_CHANNEL, SERVER_CHANNEL, Hp, GameMode, configs::GameConfigs};
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use bevy_octopus::{
    prelude::*,
    transports::{tcp::TcpAddress, udp::UdpAddress},
};
use super::player::Player;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    health: f32,
}

pub fn setup_system(
    mut commands: Commands,
    game_configs: Res<GameConfigs>
) {
    let client_address: String = format!("{}:{}", game_configs.network.ip, game_configs.network.client.port);
    let server_address: String = format!("{}:{}", game_configs.network.ip, game_configs.network.server.port);

    // Server
    commands.spawn((
        StateScoped(GameMode::Versus),
        NetworkBundle::new(SERVER_CHANNEL),
        ServerNode(TcpAddress::new(&server_address)),
    ));

    // Client
    commands.spawn((
        StateScoped(GameMode::Versus),
        NetworkBundle::new(CLIENT_CHANNEL),
        ClientNode(TcpAddress::new(&client_address)),
    ));
}

pub fn update_system(
    mut channel_received: EventReader<ReceiveChannelMessage<PlayerInfo>>,
    mut ev: EventWriter<SendChannelMessage<PlayerInfo>>,
    query: Query<&Hp, With<Player>>,
) {
    for hp in query.iter() {
        let player_info: PlayerInfo = PlayerInfo {
            health: hp.rest,
        };

        ev.write(SendChannelMessage {
            channel_id: SERVER_CHANNEL,
            message: player_info,
        });
    }

    for event in channel_received.read() {
        info!("Received: {:?}", event);
    }
}

pub fn internet_observer(trigger: Trigger<NetworkEvent>) {
    info!("trigger {:?}", trigger.event());
}
