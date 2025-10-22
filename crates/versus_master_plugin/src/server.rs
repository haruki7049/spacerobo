use super::entities::player::{Player, gun::bullet::Bullet};
use aeronet::io::{
    Session,
    connection::{Disconnected, LocalAddr},
    server::{Closed, Server},
};
use aeronet_webtransport::{
    cert,
    server::{ServerConfig, SessionRequest, SessionResponse, WebTransportServer},
};
use avian3d::prelude::*;
use bevy::prelude::*;
use chrono::{DateTime, Utc};
use spacerobo_commons::{
    BulletInformation, Information, OpponentResource, PlayerInformation, configs::GameConfigs,
};
use std::{net::IpAddr, time::Duration};

pub fn setup_system(mut commands: Commands, configs: Res<GameConfigs>) {
    let addresses: Vec<String> = configs
        .network
        .server()
        .ipaddr()
        .iter()
        .map(|v: &IpAddr| v.to_string())
        .collect();

    let identity = wtransport::Identity::self_signed(addresses)
        .expect("all given SANs should be valid DNS names");
    let cert = &identity.certificate_chain().as_slice()[0];
    let spki_fingerprint = cert::spki_fingerprint_b64(cert).expect("should be a valid certificate");
    let cert_hash = cert::hash_to_b64(cert.hash());

    info!("****************************");
    info!("SPKI FINGERPRINT");
    info!("    {spki_fingerprint}");
    info!("CERTIFICATE HASH");
    info!("    {cert_hash}");
    info!("****************************");

    let config = server_config(identity);
    commands
        .spawn_empty()
        .queue(WebTransportServer::open(config));
}

fn server_config(identity: wtransport::Identity) -> ServerConfig {
    wtransport::ServerConfig::builder()
        .with_bind_default(25571)
        .with_identity(identity)
        .keep_alive_interval(Some(Duration::from_secs(5)))
        .max_idle_timeout(Some(Duration::from_secs(5)))
        .expect("should be a valid idle timeout")
        .build()
}

pub fn on_opened(trigger: Trigger<OnAdd, Server>, servers: Query<&LocalAddr>) {
    let server = trigger.target();
    let local_addr = servers
        .get(server)
        .expect("spawned session entity should have a name");
    info!("{server} opened on {}", **local_addr);
}

pub fn on_closed(trigger: Trigger<Closed>) {
    panic!("server closed: {:?}", trigger.event());
}

pub fn on_session_request(mut request: Trigger<SessionRequest>, clients: Query<&ChildOf>) {
    let client = request.target();
    let Ok(&ChildOf(server)) = clients.get(client) else {
        return;
    };

    info!("{client} connecting to {server} with headers:");
    for (header_key, header_value) in &request.headers {
        info!("  {header_key}: {header_value}");
    }

    request.respond(SessionResponse::Accepted);
}

pub fn on_connected(trigger: Trigger<OnAdd, Session>, clients: Query<&ChildOf>) {
    let client = trigger.target();
    let Ok(&ChildOf(server)) = clients.get(client) else {
        return;
    };

    info!("{client} connected to {server}");
}

pub fn on_disconnected(trigger: Trigger<Disconnected>, clients: Query<&ChildOf>) {
    let client = trigger.target();
    let Ok(&ChildOf(server)) = clients.get(client) else {
        return;
    };

    match &*trigger {
        Disconnected::ByUser(reason) => {
            info!("{client} disconnected from {server} by user: {reason}");
        }
        Disconnected::ByPeer(reason) => {
            info!("{client} disconnected from {server} by peer: {reason}");
        }
        Disconnected::ByError(err) => {
            warn!("{client} disconnected from {server} due to error: {err:?}");
        }
    }
}

pub fn update_system(
    mut sessions: Query<(Entity, &mut Session), With<ChildOf>>,
    players_query: Query<(&Transform, &AngularVelocity, &LinearVelocity), With<Player>>,
    bullets_query: Query<(&Transform, &AngularVelocity, &LinearVelocity), With<Bullet>>,
    mut opponent_resource: ResMut<OpponentResource>,
) {
    for (client, mut session) in sessions.iter_mut() {
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

        // Player
        let mut player: Option<PlayerInformation> = None;
        for (transform, angular, linear) in players_query.iter() {
            let timestamp: DateTime<Utc> = Utc::now();

            if player.is_some() {
                panic!(
                    "Failed to create PlayerInformation for one player entity. There are two player entities"
                );
            };

            player = Some(PlayerInformation {
                transform: *transform,
                angular: *angular,
                linear: *linear,
                timestamp,
            });
        }

        // Create Information
        let information: Information = Information {
            bullets: bullets.clone(),
            player,
        };

        // Sending...
        let reply: String =
            serde_json::to_string(&information).expect("Failed to parse Information to Json data");
        info!("{client} < {reply}");
        session.send.push(reply.into());

        // Receiving...
        for packet in session.recv.drain(..) {
            let received: String =
                String::from_utf8(packet.payload.into()).unwrap_or_else(|_| "(not UTF-8)".into());
            let info: Information =
                serde_json::from_str(&received).expect("Failed to parse Json data to Information");

            debug!("{client} > {info:?}");
            opponent_resource.set(info);
        }
    }
}
