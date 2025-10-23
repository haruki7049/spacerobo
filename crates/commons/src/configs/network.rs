//! Network Configuration

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use url::Url;

/// Configuration struct
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    server: ServerConfig,
    client: ClientConfig,
}

impl Config {
    pub fn server(&self) -> ServerConfig {
        self.server.clone()
    }

    pub fn client(&self) -> ClientConfig {
        self.client.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    ipaddr: Vec<IpAddr>,
}

impl ServerConfig {
    pub fn ipaddr(&self) -> Vec<IpAddr> {
        self.ipaddr.clone()
    }
}

impl std::default::Default for ServerConfig {
    fn default() -> Self {
        Self {
            ipaddr: vec![
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                IpAddr::V6(Ipv6Addr::LOCALHOST),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientConfig {
    cert_hash: String,
    domain: Url,
}

impl ClientConfig {
    pub fn cert_hash(&self) -> String {
        self.cert_hash.clone()
    }

    pub fn domain(&self) -> Url {
        self.domain.clone()
    }
}

impl std::default::Default for ClientConfig {
    fn default() -> Self {
        Self {
            cert_hash: String::default(),
            domain: Url::parse("https://127.0.0.1:25571").unwrap(),
        }
    }
}
