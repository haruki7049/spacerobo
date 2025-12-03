use aeronet::io::{
    Session,
    connection::{Disconnected, LocalAddr},
    server::{Closed, Server},
};
use aeronet_webtransport::{
    cert,
    server::{ServerConfig, SessionRequest, SessionResponse, WebTransportServer},
};
use bevy::prelude::*;
use std::{net::IpAddr, time::Duration};

// Re-export types that will be needed by plugins
// ChildOf is already available from bevy::prelude

/// Setup system for creating and starting a WebTransport server
pub fn setup_system(mut commands: Commands, server_config: Res<ServerConfigResource>) {
    let addresses: Vec<String> = server_config
        .ipaddr
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

    let config = server_config_builder(identity, server_config.port);
    commands
        .spawn_empty()
        .queue(WebTransportServer::open(config));
}

fn server_config_builder(identity: wtransport::Identity, port: u16) -> ServerConfig {
    wtransport::ServerConfig::builder()
        .with_bind_default(port)
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

/// Resource to configure the server
#[derive(Resource)]
pub struct ServerConfigResource {
    pub ipaddr: Vec<IpAddr>,
    pub port: u16,
}

impl Default for ServerConfigResource {
    fn default() -> Self {
        Self {
            ipaddr: vec![IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)],
            port: 25571,
        }
    }
}
