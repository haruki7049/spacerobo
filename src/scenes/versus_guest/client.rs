use super::entities::{
    opponent::{DamageCollector, Opponent},
    player::{Player, gun::bullet::Bullet},
};
use crate::{
    BulletInformation, Damage, Information, OpponentResource, PlayerInformation,
    configs::GameConfigs,
};
use aeronet::io::{Session, SessionEndpoint, connection::Disconnected};
use aeronet_webtransport::{
    cert,
    client::{ClientConfig, WebTransportClient},
};
use avian3d::prelude::*;
use bevy::prelude::*;
use chrono::{DateTime, Utc};
use url::Url;

pub fn setup_system(
    mut commands: Commands,
    configs: Res<GameConfigs>,
    mut session_id: Local<usize>,
) {
    let target: Url = configs.network.client.domain.clone();
    *session_id += 1;
    let name: String = format!("{}. {}", *session_id, target);

    let cert_hash: String = configs.network.client.cert_hash.clone();
    let config: ClientConfig = match client_config(cert_hash) {
        Ok(config) => config,
        Err(err) => {
            panic!("Failed to create client config: {err:?}");
        }
    };

    commands
        .spawn(Name::new(name))
        .queue(WebTransportClient::connect(config, target.to_string()));
}

fn client_config(cert_hash: String) -> Result<ClientConfig, Box<dyn std::error::Error>> {
    use {aeronet_webtransport::wtransport::tls::Sha256Digest, core::time::Duration};

    let config = ClientConfig::builder().with_bind_default();

    let config = if cert_hash.is_empty() {
        config.with_server_certificate_hashes([])
    } else {
        let hash = cert::hash_from_b64(&cert_hash)?;
        config.with_server_certificate_hashes([Sha256Digest::new(hash)])
    };

    Ok(config
        .keep_alive_interval(Some(Duration::from_secs(1)))
        .max_idle_timeout(Some(Duration::from_secs(5)))
        .expect("should be a valid idle timeout")
        .build())
}

pub fn on_connecting(trigger: Trigger<OnAdd, SessionEndpoint>) {
    let target = trigger.target();

    info!("{target} connecting");
}

pub fn on_connected(trigger: Trigger<OnAdd, Session>) {
    let target = trigger.target();

    info!("{target} connected");
}

pub fn on_disconnected(trigger: Trigger<Disconnected>) {
    let target = trigger.target();

    let messages = match &*trigger {
        Disconnected::ByUser(reason) => {
            format!("{target} disconnected by user: {reason}")
        }
        Disconnected::ByPeer(reason) => {
            format!("{target} disconnected by peer: {reason}")
        }
        Disconnected::ByError(err) => {
            format!("{target} disconnected due to error: {err:?}")
        }
    };

    info!("{messages}");
}

pub fn update_system(
    mut sessions: Query<(Entity, &mut Session)>,
    mut opponent_resource: ResMut<OpponentResource>,
    player: Query<(&Transform, &AngularVelocity, &LinearVelocity), With<Player>>,
    bullets_query: Query<(&Transform, &AngularVelocity, &LinearVelocity), With<Bullet>>,
    damage_collector_query: Query<&DamageCollector, With<Opponent>>,
) {
    for (server, mut session) in sessions.iter_mut() {
        let session = &mut *session;

        // Bullets
        let mut bullets: Vec<BulletInformation> = Vec::new();
        for (transform, angular, linear) in bullets_query.iter() {
            let b: BulletInformation = BulletInformation {
                transform: *transform,
                angular: *angular,
                linear: *linear,
            };

            bullets.push(b);
        }

        let mut damages: Vec<Damage> = Vec::new();
        for damage_collector in damage_collector_query.iter() {
            for inner in &damage_collector.0 {
                damages.push(inner.clone());
            }
        }

        // Player
        for (transform, angular, linear) in player.iter() {
            let timestamp: DateTime<Utc> = Utc::now();
            let information: Information = Information {
                bullets: bullets.clone(),
                player: PlayerInformation {
                    damages: damages.clone(),
                    transform: *transform,
                    angular: *angular,
                    linear: *linear,
                    timestamp,
                },
            };

            let reply: String = serde_json::to_string(&information)
                .expect("Failed to parse Information to Json data");
            info!("{server} < {reply}");
            session.send.push(reply.into());
        }

        for packet in session.recv.drain(..) {
            let received: String =
                String::from_utf8(packet.payload.into()).unwrap_or_else(|_| "(not UTF-8)".into());
            let info: Information =
                serde_json::from_str(&received).expect("Failed to parse Json data to Information");

            info!("{server} > {info:?}");
            opponent_resource.set(info);
        }
    }
}
