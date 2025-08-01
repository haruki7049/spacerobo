use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ip: IpAddr,
    pub client: Client,
    pub server: Server,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            client: Client::default(),
            server: Server::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub port: u16,
}

impl std::default::Default for Client {
    fn default() -> Self {
        Self {
            port: 10000,
        }
    }
}

impl std::default::Default for Server {
    fn default() -> Self {
        Self {
            port: 10001,
        }
    }
}
