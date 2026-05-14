//! Per-cluster LAN and DNS-resolver typed records.
//!
//! Replaces the `10.18.0.0/24` / `10.18.0.1` / Cloudflare/Quad9 style
//! literals scattered across CriomOS networking modules. Per
//! `~/primary/reports/system-specialist/119-horizon-data-needed-to-purge-criomos-literals.md`
//! §§2-3 and the closed `br-lan stays in CriomOS-lib` decision in
//! `~/primary/reports/system-assistant/14-horizon-re-engineering-ready-state.md` §7.

use ipnet::IpNet;
use nota_codec::{NotaDecode, NotaEncode, NotaRecord};
use serde::{Deserialize, Serialize};

use crate::address::IpAddress;
use crate::error::{Error, Result};

/// CIDR notation for a LAN segment, e.g. `10.18.0.0/24`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct LanCidr(IpNet);

impl LanCidr {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        s.parse()
            .map(Self)
            .map_err(|e| Error::InvalidLanCidr { got: s, source: e })
    }

    pub fn ipnet(self) -> IpNet {
        self.0
    }
}

impl TryFrom<String> for LanCidr {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl From<LanCidr> for String {
    fn from(c: LanCidr) -> Self {
        c.0.to_string()
    }
}

impl std::fmt::Display for LanCidr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl NotaEncode for LanCidr {
    fn encode(&self, encoder: &mut nota_codec::Encoder) -> nota_codec::Result<()> {
        encoder.write_string(&self.0.to_string())
    }
}

impl NotaDecode for LanCidr {
    fn decode(decoder: &mut nota_codec::Decoder<'_>) -> nota_codec::Result<Self> {
        let s = decoder.read_string()?;
        LanCidr::try_new(s.clone()).map_err(|e| nota_codec::Error::Validation {
            type_name: "LanCidr",
            message: format!("invalid LanCidr {s:?}: {e}"),
        })
    }
}

/// Per-cluster LAN configuration: subnet, gateway, DHCP pool, lease
/// behaviour. The bridge interface name is a CriomOS implementation
/// constant and stays in `CriomOS-lib` (closed decision in report 14
/// §7 / row 8); other clusters that diverge can promote it later.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct LanNetwork {
    pub cidr: LanCidr,
    pub gateway: IpAddress,
    pub dhcp_pool: DhcpPool,
    pub lease_policy: LeasePolicy,
}

/// DHCP address pool — the inclusive range of IPs the LAN's DHCP
/// server hands out to clients.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct DhcpPool {
    pub start: IpAddress,
    pub end: IpAddress,
}

/// DHCP lease behaviour. Minimal today (default TTL only); future
/// fields (max TTL, sticky reservations, refresh policy) extend at
/// the tail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct LeasePolicy {
    /// Default lease lifetime in seconds. dnsmasq's `--dhcp-range`
    /// lease-time argument is rendered from this.
    pub default_ttl_seconds: u32,
}

/// Per-cluster DNS resolver policy. Replaces the Cloudflare/Quad9
/// literals in `network/default.nix`, `network/resolver.nix`,
/// `network/dnsmasq.nix`, `network/networkd.nix`.
///
/// Tailnet MagicDNS (`100.100.100.100`) is not modelled here — that
/// belongs to the tailnet controller record (step 11 territory).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ResolverPolicy {
    /// Primary upstream resolvers, queried first.
    pub upstreams: Vec<IpAddress>,
    /// Fallback resolvers when upstreams fail.
    pub fallbacks: Vec<IpAddress>,
    /// Local addresses the cluster's DNS service binds to (loopback,
    /// LAN gateway IP, etc.). Consumers (dnsmasq, systemd-resolved,
    /// networkd) render listen directives from this.
    pub listens: Vec<IpAddress>,
}
