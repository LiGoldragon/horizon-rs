//! Projected LAN and DNS-resolver typed records.
//!
//! These records are not authored in a cluster proposal. Horizon derives
//! them from the pan-horizon LAN pool plus cluster/router identity.

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

    pub fn as_ipnet(&self) -> IpNet {
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

/// Projected LAN configuration: subnet, gateway, DHCP pool. Lease
/// behaviour is a CriomOS runtime default, not Horizon data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct LanNetwork {
    pub cidr: LanCidr,
    pub gateway: IpAddress,
    pub dhcp_pool: DhcpPool,
}

/// DHCP address pool — the inclusive range of IPs the LAN's DHCP
/// server hands out to clients.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct DhcpPool {
    pub start: IpAddress,
    pub end: IpAddress,
}

/// Derived local DNS listen addresses. Upstream and fallback resolvers
/// are CriomOS defaults, not Horizon data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ResolverPolicy {
    /// Local addresses the cluster's DNS service binds to (loopback,
    /// LAN gateway IP, etc.).
    pub listens: Vec<IpAddress>,
}
