//! Network Configuration

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Configuration struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub ipaddr: Vec<IpAddr>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            ipaddr: vec![
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                IpAddr::V6(Ipv6Addr::LOCALHOST),
            ],
        }
    }
}
