use aeronet_webtransport::server::WebTransportServerPlugin;
use bevy::{log::LogPlugin, prelude::*};
use spacerobo_commons::configs::GameConfigs;
use aeronet_webtransport::server::WebTransportServer;
use aeronet_webtransport::cert;
use std::{net::IpAddr, time::Duration};

fn main() {
    let configs: GameConfigs = GameConfigs::default();

    App::new()
        // Use MinimalPlugins for a headless server and add LogPlugin
        .add_plugins((LogPlugin::default(), MinimalPlugins))
        // Add the WebTransport server plugin
        .add_plugins(WebTransportServerPlugin)
        .insert_resource(configs)
        .add_systems(Startup, setup_system)
        // TODO: Add actual server game logic and communication systems here
        .run();
}

// Logic migrated from client/src/plugins/versus_master_plugin/server.rs
fn setup_system(mut commands: Commands, configs: Res<GameConfigs>) {
    // Get IP addresses from config
    let addresses: Vec<String> = configs
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
