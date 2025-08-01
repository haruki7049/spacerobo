pub mod opponent;

use crate::{CLIENT_CHANNEL, GameMode, SERVER_CHANNEL, configs::GameConfigs};
use bevy::prelude::*;
use bevy_octopus::{prelude::*, transports::tcp::TcpAddress};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    pub health: f32,
    pub transform: Transform,
    pub linear: Vec3,
    pub angular: Vec3,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSpawnInfo {
    pub health: f32,
    pub transform: Transform,
}

pub fn setup_system(
    mut commands: Commands,
    mut ev: EventWriter<SendChannelMessage<PlayerSpawnInfo>>,
    game_configs: Res<GameConfigs>,
) {
    let client_address: String = format!(
        "{}:{}",
        game_configs.network.ip, game_configs.network.client.port
    );
    let server_address: String = format!(
        "{}:{}",
        game_configs.network.ip, game_configs.network.server.port
    );

    ev.write(SendChannelMessage {
        channel_id: CLIENT_CHANNEL,
        message: PlayerSpawnInfo {
            health: 100.0,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        },
    });

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

pub fn internet_observer(trigger: Trigger<NetworkEvent>) {
    info!("trigger {:?}", trigger.event());
}
