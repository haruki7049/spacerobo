use aeronet::io::{
    Session,
    connection::{Disconnected, LocalAddr},
    server::{Closed, Server},
};
use aeronet_webtransport::{
    cert,
    server::{
        SessionRequest, SessionResponse, WebTransportServer, WebTransportServerPlugin,
    },
};
use bevy::{log::LogPlugin, prelude::*};
use spacerobo_commons::configs::GameConfigs;
use std::{net::IpAddr, time::Duration};

fn main() {
    let configs: GameConfigs = GameConfigs::default();

    App::new()
        .add_plugins((LogPlugin::default(), MinimalPlugins))
        .add_plugins(WebTransportServerPlugin)
        .insert_resource(configs)
        .add_systems(Startup, setup_system)
        .add_observer(on_opened)
        .add_observer(on_closed)
        .add_observer(on_session_request)
        .add_observer(on_connected)
        .add_observer(on_disconnected)
        .run();
}

// Logic migrated from client/src/plugins/versus_master_plugin/server.rs
fn setup_system(mut commands: Commands, game_configs: Res<GameConfigs>) {
    // Get IP addresses from config
    let addresses: Vec<String> = game_configs
        .network()
        .server()
        .ipaddr()
        .iter()
        .map(|v: &IpAddr| v.to_string())
        .collect();

    // Generate self-signed certificate (for testing/development)
    let identity = wtransport::Identity::self_signed(addresses)
        .expect("all given SANs should be valid DNS names");
    let cert = &identity.certificate_chain().as_slice()[0];
    let spki_fingerprint = cert::spki_fingerprint_b64(cert).expect("should be a valid certificate");
    let cert_hash = cert::hash_to_b64(cert.hash());

    info!("****************************");
    info!("SPKI FINGERPRINT: {spki_fingerprint}");
    info!("CERTIFICATE HASH: {cert_hash}");
    info!("****************************");

    // Server configuration
    let config = aeronet_webtransport::server::ServerConfig::builder()
        .with_bind_default(25571)
        .with_identity(identity)
        .keep_alive_interval(Some(Duration::from_secs(5)))
        .max_idle_timeout(Some(Duration::from_secs(5)))
        .expect("should be a valid idle timeout")
        .build();

    // Spawn the WebTransport Server
    commands
        .spawn_empty()
        .queue(WebTransportServer::open(config));
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
