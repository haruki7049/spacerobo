//! Network Configuration

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use url::Url;

/// Configuration struct
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub client: ClientConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub ipaddr: Vec<IpAddr>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    pub cert_hash: String,
    pub domain: Url,
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

impl std::default::Default for ClientConfig {
    fn default() -> Self {
        Self {
            cert_hash: String::default(),
            domain: Url::parse("https://127.0.0.1:25571").unwrap(),
        }
    }
}
