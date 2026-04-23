//! Network addresses: yggdrasil identifiers, node IPs, link-local
//! per-interface addresses.

use std::net::Ipv6Addr;

use ipnet::IpNet;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Yggdrasil-mesh IPv6 address. Always within `200::/7`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct YggAddress(Ipv6Addr);

impl YggAddress {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        s.parse()
            .map(Self)
            .map_err(|e| Error::InvalidYggAddress { got: s, source: e })
    }

    pub fn ipv6(self) -> Ipv6Addr {
        self.0
    }
}

impl TryFrom<String> for YggAddress {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl From<YggAddress> for String {
    fn from(a: YggAddress) -> Self {
        a.0.to_string()
    }
}

impl std::fmt::Display for YggAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Yggdrasil subnet identifier (e.g. `300:ca41:6b12:fba`). Free-form
/// today — not a parsed CIDR — because the legacy data carries it as
/// the bare prefix without a prefix length. Promote to `Ipv6Net` when
/// goldragon emits canonical CIDRs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YggSubnet(String);

impl YggSubnet {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            Err(Error::InvalidYggAddress {
                got: s,
                source: "::".parse::<Ipv6Addr>().unwrap_err(),
            })
        } else {
            Ok(Self(s))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Internal-cluster routing IP, as a CIDR.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct NodeIp(IpNet);

impl NodeIp {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        s.parse()
            .map(Self)
            .map_err(|e| Error::InvalidNodeIp { got: s, source: e })
    }

    pub fn ipnet(self) -> IpNet {
        self.0
    }
}

impl TryFrom<String> for NodeIp {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl From<NodeIp> for String {
    fn from(a: NodeIp) -> Self {
        a.0.to_string()
    }
}

/// Network interface name (`enp0s25`, `wlp3s0`, …). Hardware-dependent;
/// the proposal author specifies it per link-local entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Iface(String);

impl Iface {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Iface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Raw input form of a link-local address: an interface plus a
/// 64-bit suffix. Renders as `fe80::<suffix>%<iface>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkLocalIp {
    pub iface: Iface,
    pub suffix: String,
}

impl LinkLocalIp {
    pub fn render(&self) -> LinkLocalAddress {
        LinkLocalAddress(format!("fe80::{}%{}", self.suffix, self.iface))
    }
}

/// Projected (rendered) link-local address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LinkLocalAddress(String);

impl LinkLocalAddress {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for LinkLocalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
