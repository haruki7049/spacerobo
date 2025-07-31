use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ip: IpAddr,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }
}
