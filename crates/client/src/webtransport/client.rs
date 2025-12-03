use aeronet::io::{Session, SessionEndpoint, connection::Disconnected};
use aeronet_webtransport::{
    cert,
    client::{ClientConfig, WebTransportClient},
};
use bevy::prelude::*;
use url::Url;

/// Setup system for creating and connecting a WebTransport client
pub fn setup_system(
    mut commands: Commands,
    client_config: Res<ClientConfigResource>,
    mut session_id: Local<usize>,
) {
    let target: Url = client_config.domain.clone();
    *session_id += 1;
    let name: String = format!("{}. {}", *session_id, target);

    let cert_hash: String = client_config.cert_hash.clone();
    let config: ClientConfig = match client_config_builder(cert_hash) {
        Ok(config) => config,
        Err(err) => {
            panic!("Failed to create client config: {err:?}");
        }
    };

    commands
        .spawn(Name::new(name))
        .queue(WebTransportClient::connect(config, target.to_string()));
}

fn client_config_builder(cert_hash: String) -> Result<ClientConfig, Box<dyn std::error::Error>> {
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

/// Resource to configure the client
#[derive(Resource)]
pub struct ClientConfigResource {
    pub domain: Url,
    pub cert_hash: String,
}

impl Default for ClientConfigResource {
    fn default() -> Self {
        Self {
            domain: Url::parse("https://localhost:25571").expect("should be a valid URL"),
            cert_hash: String::new(),
        }
    }
}
