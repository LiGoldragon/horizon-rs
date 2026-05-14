//! WireGuard peers that act as outbound proxies.
//!
//! A peer on the `wgProxies` interface; downstream nix module routes
//! `0.0.0.0/0` through it. One per VPN connection (NordVPN, etc.).

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::address::NodeIp;
use crate::pub_key::WireguardPubKey;

/// An external WireGuard proxy this node tunnels through.
#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct WireguardProxy {
    pub pub_key: WireguardPubKey,
    /// `host:port` form.
    pub endpoint: String,
    /// Address assigned to our wireguard interface for this proxy.
    pub interface_ip: NodeIp,
}
